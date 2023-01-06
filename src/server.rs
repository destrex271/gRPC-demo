use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloRequest, HelloResponse};

pub mod hello_world{
    tonic::include_proto!("helloworld"); // calling in the proto package
}

#[derive(Debug, Default)]
pub struct MyGreeter{}

#[tonic::async_trait]
impl Greeter for MyGreeter{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>
    ) -> Result<Response<HelloResponse>, Status>
    {
        println!("Request received : {:?}", request);
        let reply = hello_world::HelloResponse{
            message: format!("Hello there : {}!", request.into_inner().name)
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
