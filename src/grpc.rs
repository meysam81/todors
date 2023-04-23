pub use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};

pub use proto::healthcheck::health_check_server::{HealthCheck, HealthCheckServer};
use proto::healthcheck::{Ping, Pong};

mod proto {
    pub mod healthcheck {
        tonic::include_proto!("healthcheck");
    }
}

#[derive(Debug, Default)]
pub struct TodoHealthCheck {}

#[tonic::async_trait]
impl HealthCheck for TodoHealthCheck {
    async fn check(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        println!("Got a request: {:?}", request);

        let reply = proto::healthcheck::Pong {
            message: "pong".to_string(),
        };

        Ok(Response::new(reply))
    }
}
