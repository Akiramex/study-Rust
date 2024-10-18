use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer}, 
    HelloRequest, HelloResponse, 
    VotingRequest, VotingResponse
};

use greet::{
    greeter_server::{Greeter, GreeterServer},
    HelloReq, HelloResp
};

use tracing::info;

pub mod voting {
    include!("../proto/voting.rs");
}
pub mod greet {
    include!("../proto/greet.rs");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    // Voting是服务名,需要实现vote这个trait(方法)
    async fn vote(
        &self,
        request: Request<VotingRequest>,
    ) -> Result<Response<VotingResponse>, Status> {
        info!("{request:?}");
        let r: &VotingRequest = request.get_ref();
        match r.vote {
            0 => Ok(Response::new(voting::VotingResponse{
                confirmation: format!("upvoted for {}", r.url),
            })),
            1 => Ok(Response::new(voting::VotingResponse{
                confirmation: format!("downvoted for {}", r.url)
            })),
            _ => Err(Status::new(tonic::Code::OutOfRange, "Invailed vote provided"))
        }
    }

    async fn hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        info!("{request:?}");
        let r: &HelloRequest = request.get_ref();
        Ok(Response::new(voting::HelloResponse{
            message: format!("Hello {}", r.name)
        }))
    }
}


// 类型和变量都是本身
#[derive(Debug)]
pub struct GreetService;

#[tonic::async_trait]
impl Greeter for GreetService {
    async fn say_hello(&self, req: Request<HelloReq>) -> Result<Response<HelloResp>, Status> {
        info!("{req:?}");
        let hello_str = req.into_inner().content;
        info!("greet from client: {}", hello_str);
        Ok(Response::new(HelloResp { message: hello_str }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 当前项目
    tracing_subscriber::fmt().init();

    // 全局的日志 会打印好多别的库的信息
    //use tracing::{info, Level};
    //use tracing_subscriber::FmtSubscriber;
    //let subscriber = FmtSubscriber::builder()
    //.with_max_level(Level::TRACE)
    //.finish();
    
    //tracing::subscriber::set_global_default(subscriber)
    //.expect("setting default subscriber failed");

    let address = "[::1]:50051".parse().unwrap();

    let voting_server = VotingService::default();

    Server::builder()
        .add_service(VotingServer::new(voting_server))
        .add_service(GreeterServer::new(GreetService))
        .serve(address)
        .await?;

    Ok(())
}