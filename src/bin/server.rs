use tonic::{transport::Server,Request, Response, Status};
use crate::sample::sample_service_server::{SampleService, SampleServiceServer};
use crate::sample::{SampleRequest, SampleResponse};
use tracing::info;

pub mod sample {
    tonic::include_proto!("sample");
}

#[derive(Debug, Default)]
pub struct MySampleService {}

#[tonic::async_trait]
impl SampleService for MySampleService {
    async fn sample(&self, request: Request<SampleRequest>) -> Result<Response<SampleResponse>, Status> {
        println!("Got a request: {:?}", request);

        let sample = SampleResponse {
            message: format!("Echo {}", request.into_inner().name),
        };

        Ok(Response::new(sample))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .json()
        .init();

    let addr = "[::1]:50051".parse()?;
    let sample_service = MySampleService::default();

    info!(message = "server listening at 50051");

    Server::builder()
        .add_service(SampleServiceServer::new(sample_service))
        .serve(addr)
        .await?;

    Ok(())
}