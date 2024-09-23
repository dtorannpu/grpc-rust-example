use crate::sample::sample_service_client::SampleServiceClient;
use crate::sample::SampleRequest;

pub mod sample {
    tonic::include_proto!("sample");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SampleServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SampleRequest{
        name: "Tonic".into(),
    });

    let response = client.sample(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}