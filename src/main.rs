#![deny(clippy::all)]
use envoy::service::auth::v3::authorization_server::{Authorization, AuthorizationServer};
use envoy::service::auth::v3::{CheckRequest, CheckResponse};
use google::rpc;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{instrument, Level};

pub mod envoy {
    pub mod config {
        pub mod core {
            pub mod v3 {
                tonic::include_proto!("envoy.config.core.v3");
            }
        }
    }
    pub mod r#type {
        pub mod v3 {
            tonic::include_proto!("envoy.r#type.v3");
        }
    }
    pub mod service {
        pub mod auth {
            pub mod v3 {
                tonic::include_proto!("envoy.service.auth.v3");
            }
        }
    }
}
pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}
pub mod xds {
    pub mod core {
        pub mod v3 {
            tonic::include_proto!("xds.core.v3");
        }
    }
}

#[derive(Debug, Default)]
pub struct MyAuthz {}


#[tonic::async_trait]
impl Authorization for MyAuthz {
    #[instrument]
    async fn check(&self, _request: Request<CheckRequest>) -> Result<Response<CheckResponse>, Status> {
        let response = CheckResponse{
            status: Some(rpc::Status{
                code: 200,
                details: vec![],
                message: "".into(),
            }),
            dynamic_metadata: None,
            http_response: None,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let addr = "[::1]:50051".parse()?;
    let authz = MyAuthz::default();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
        .set_serving::<AuthorizationServer<MyAuthz>>()
        .await;

    Server::builder()
        .add_service(AuthorizationServer::new(authz))
        .add_service(health_service)
        .serve(addr)
        .await?;
    Ok(())
}