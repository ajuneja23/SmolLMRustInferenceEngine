use smollm::smollm_client::{SmollmClient};
use smollm::{SmolLmReq};
use tonic::transport::Channel;
use std::io;
use tokio::sync::{mpsc,Mutex};
use std::sync::{Arc};
use tokio::time::{sleep, Duration}; // Add this line


use chrono::Utc;

use std::{env,fs};
use colored::Colorize;

pub mod smollm {
    tonic::include_proto!("smollm");
}

async fn createClient(port: &u32)->Result<SmollmClient<Channel>,Box<dyn std::error::Error>>  {
    return Ok(SmollmClient::connect(format!("http://[::1]:{}",port)).await?);
}


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut active=Arc::new(Mutex::new(Vec::from([0,0,0,0,0])));
    let ports: [u32;5]=[8080,8081,8082,8083,8084];
    let mut clientsVec=Vec::new();
    for port in ports.iter() {
        clientsVec.push(Arc::new(Mutex::new(createClient(port).await?)));
    }
    let mut transmitters: Vec<mpsc::Sender<String>>=Vec::new();
    let mut receivers=Vec::new();
    for i in 0..5 {
        let (tx,rx)=mpsc::channel(16);
        transmitters.push(tx);
        receivers.push(Arc::new(Mutex::new(rx)));
    }
    for i in 0..5 {
        let mut rx=Arc::clone(&receivers[i]);
        let mut activeClone=Arc::clone(&active);
        let mut client=Arc::clone(&clientsVec[i]);
        tokio::spawn(async move {
            loop {
                let mut thisRx=rx.lock().await;
                let mut thisClient=client.lock().await;
                let lmPrompt=thisRx.recv().await.unwrap();
                println!("Process handler received prompt: {}",lmPrompt.blue());
        
                match (thisClient.send_req(SmolLmReq {
                    prompt: lmPrompt
                })).await {
                    Ok(mut res_stream)=> {
                        let stream=res_stream.get_mut();
                        let mut total_response=String::new();
                        loop {
                            match stream.message().await {
                                Ok(Some(res))=> {
                                    println!("{}", format!("Received response {:?} at time {} from node {}", res, Utc::now().to_rfc3339(), i).green());
                                    if res.cur_token=="DONE STREAMING RESPONSE" {
                                        let mut active_lock=activeClone.lock().await;
                                        active_lock[i]-=1;
                                        println!("{}",format!("Channel {} has {} active jobs",i,active_lock[i]).purple());
                                        break;
                                    }
                                    total_response.push_str(&res.cur_token);
                                },
                                Ok(None)=>break,
                                Err(e)=>{ 
                                    println!("received error: {}",e);
                                    break;
                                }
                                
                            }

                        }
                        println!("{}",format!("final response: {}",total_response).cyan());
                    },
                    Err(e)=> {
                        println!("error sending request: {:?}",e);
                        let mut active_lock=activeClone.lock().await;
                        active_lock[i]-=1;
                    }
                }
            }
        });
    }
    let contents=fs::read_to_string("./src/jobs.txt").expect("failed to read jobs.txt");
    println!("{contents}");
    let mut k=0;
    let prompts=contents.split("\n");
    for prompt in prompts {
        sleep(Duration::from_secs(1)).await;
        k+=1;
        println!("Prompt {}: {} IS BEING PROCESSED",k,prompt);
        let mut i=0;
        let mut active_lock=active.lock().await;
        while i<5 && active_lock[i] != 0 {
            i+=1;
        }
        if i !=5 {
            active_lock[i]+=1;
            println!("{}",format!("Channel {} has {} active jobs",i,active_lock[i]).bright_blue());
            drop(active_lock);
            transmitters[i].send(String::from(prompt)).await.unwrap();
        } else {
            let mut minInd=0;
            let mut minVal=active_lock[0];
            for j in 1..5 {
                if active_lock[j]<minVal {
                    minInd=j;
                    minVal=active_lock[j];
                }
            }
            active_lock[minInd]+=1;
            println!("{}",format!("Channel {} has {} active jobs",minInd,active_lock[minInd]).yellow());
            drop(active_lock);
            transmitters[minInd].send(String::from(prompt)).await.unwrap();
        }
    }
    loop {
        sleep(Duration::from_secs(1)).await;
        let active_lock=active.lock().await;
        let total=active_lock.iter().sum::<i32>();
        if total == 0 {
            break;
        }
        println!("{}",format!("All jobs submitted. Waiting for {} jobs to complete",total).white());
    }
    println!("{}",format!("All jobs done").magenta());
    Ok(())

}
