use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world{
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut client = GreeterClient::connect("http://[::1]:50051").await?;


    for _ in 1..10{
        let request = tonic::Request::new(HelloRequest{
            name: "Tonic".into()
        });
        let response = client.say_hello(request).await;
        println!("Response = {:?}", response);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
