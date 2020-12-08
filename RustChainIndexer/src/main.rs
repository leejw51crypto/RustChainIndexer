mod proto;

use proto::tendermint::blockchain::{BlockRequest, BlockResponse};
use proto::cosmos::greeter_client::GreeterClient;
use proto::cosmos::base::reflection::v1beta1::{ListAllInterfacesRequest,ListImplementationsRequest};
use proto::cosmos::base::reflection::v1beta1::reflection_service_client::ReflectionServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let mut client = ReflectionServiceClient::connect("http://192.168.1.141:9091").await?;

    let request = tonic::Request::new(ListAllInterfacesRequest::default());

    let response = client.list_all_interfaces(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
