use tonic::{transport::Server, Request, Response, Status};
use computations::{CompReq,CompRes};
use computations::computations_server::{Computations, ComputationsServer};
use futures::stream::{self,Stream};
use tokio::time::{self,Duration};
use tokio_stream::StreamExt;
use tokio_stream::{wrappers::ReceiverStream};
use tokio::sync::mpsc;
use chrono::Utc;

pub mod computations {
    tonic::include_proto!("computations");
}

#[derive(Debug,Default)]
pub struct ComputationsService;



#[tonic::async_trait]
impl Computations for ComputationsService {
    type sendComputationStream=ReceiverStream<Result<CompRes,Status>>;
    async fn send_computation( 
        &self,
        request: Request<CompReq>,
    )->Result<Response<Self::sendComputationStream>, Status> {
        println!("Received a request {:?}",request);
        let req=request.into_inner();
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for i in 0..req.a.len() {
                time::sleep(Duration::from_secs(1)).await; // {{ edit_1 }}
                
                tx.send(Ok(CompRes {
                    successful: true,
                    ans: req.a[i]+req.b[i]+req.c[i],
                    message: format!("Here's the goods at {}",Utc::now().to_rfc3339())
                })).await.unwrap();
                println!("Task {} finished at time {}",i+1,Utc::now().to_rfc3339());
            }
        });//result is Ok vs Err
        Ok(Response::new(ReceiverStream::new(rx)))

        /*let response_stream=async_stream::stream! {
            let request=request.into_inner();
            for i in 0..request.a.len() {
                yield Ok(CompRes {
                    message: String::from("success"),
                    successful: true,
                    ans: request.a[i]+request.b[i]+request.c[i]
                });
            }
        };*/
    }

}

#[tokio::main] 
async fn main()->Result<(),Box<dyn std::error::Error>> {
    let addr="[::1]:50051".parse().unwrap();
    let compService=ComputationsService::default();
    Server::builder().add_service(ComputationsServer::new(compService)).serve(addr).await?;
    Ok(())

}