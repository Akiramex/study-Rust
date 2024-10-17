use tonic::{transport::Server, Request, Response, Status};
use voting::{
    voting_server::{Voting, VotingServer},
    VotingRequest,VotingResponse,
};

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod voting {
    include!("../proto/voting.rs");
}

pub mod hello {
    include!("../proto/hello.rs");
}

#[derive(Debug, Default)]
pub struct MyVotingServer {}

#[tonic::async_trait]
// Voting是服务名,需要实现vote这个trait
impl Voting for MyVotingServer {
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
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 当前项目
    tracing_subscriber::fmt().init();

    // 全局的日志
    //let subscriber = FmtSubscriber::builder()
    //.with_max_level(Level::TRACE)
    //.finish();
    
    //tracing::subscriber::set_global_default(subscriber)
    //.expect("setting default subscriber failed");

    info!("ABC");

    let address = "[::1]:50051".parse().unwrap();

    let voting_server = MyVotingServer::default();

    Server::builder()
        .add_service(VotingServer::new(voting_server))
        .serve(address)
        .await?;

    Ok(())
}