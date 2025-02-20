use computations::computations_client::{ComputationsClient};
use computations::{CompReq};
use tonic::transport::Channel;
use chrono::Utc;

pub mod computations {
    tonic::include_proto!("computations");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ComputationsClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(
        CompReq {
            a: vec![1,3,4,5,6],
            b: vec![234,6,6,32,234],
            c: vec![30,34034,34,46,3],
            description: "here's the good my g".to_owned()
        }
    );
    let mut res_stream=(client.send_computation(request).await?).into_inner();
    while let Ok(Some(res))=res_stream.message().await {
        println!("Received response {:?} at time {}",res,Utc::now().to_rfc3339());
    }


    Ok(())
}