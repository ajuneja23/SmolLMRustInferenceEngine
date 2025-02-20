use smollm::smollm_client::{SmollmClient};
use smollm::{SmolLmReq};
use tonic::transport::Channel;
use std::io;
use tokio::sync::mpsc;

use chrono::Utc;



pub mod smollm {
    tonic::include_proto!("smollm");
}

async fn createClient(port)->Result<SmollmClient<channel>,Box<std dyn::error:Error>>  {
    return SmollmClient::connect(format!("http://[::1]:{}",port)).await?;
}


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let mut active=[0;5];//num of active requests 
    let ports=[8080,8081,8082,8083,8084];
    let mut clientsVec=Vec::new();
    for port in ports.iter() {
        clientsVec.push(createClient(port).await?);
    }
    let mut channels=Vec::new();
    for i in 0..5 {
        let (tx,rx)=mpsc::channel();
        channels.push((tx,rx));//(tx,rx) for each channel
    }
    for i in 0..5 {
        let rx=channels[i].1.clone();
        let mut client=clientsVec[i].clone();
        tokio::spawn(async move {
            loop {
                let lmPrompt=rx.recv().unwrap();
                let mut res_stream=(client.send_req(SmolLmReq {
                    prompt: lmPrompt
                }));
                while Ok(Some(chunk))=res_stream.message().await {
                    println!("Received response {:?} at time {}",chunk,Utc::now().to_rfc3339());
                    if chunk.cur_token="DONE STREAMING RESPONSE" {
                        active[i]--;
                        break;
                    }
                    total_response.push_str(&chunk.cur_token);
                }
            }
        })
    }
    loop {
        let mut input=String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n)=> {//gives result with num bytes ig
                println!("{}",input);
                let mut i=0;
                while i<5 && active[i] !=0 {
                i++;
                }
                active[i]++;
                if i != 6 {
                    channels[i].send(&input).unwrap();
                } else {
                    let minInd=0;
                    let minVal=active[0];
                    for j in 1..5 {
                        if active[i]<minVal {
                            minInd=j;
                            minVal=active[j];
                        }
                    }
                    channels[minInd].send(&input).unwrap();
                }
            }
        } 
    }

    /*let mut total_response=String::new();
    let mut res_stream=(client.send_req(request).await?).into_inner();
    while let Ok(Some(res))=res_stream.message().await {
        println!("Received response {:?} at time {}",res,Utc::now().to_rfc3339());
        total_response.push_str(&res.cur_token);
    }
    println!("{}",total_response);*/
    Ok(())

}
