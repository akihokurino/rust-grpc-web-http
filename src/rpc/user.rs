use tonic::{Request, Response, Status};

use pb::user_service_server::{UserService, UserServiceServer};
use uuid::Uuid;

pub mod common {
    tonic::include_proto!("common");
}
pub mod pb {
    tonic::include_proto!("user");
}

pub fn new_server() -> UserServiceServer<UserServiceImpl> {
    UserServiceServer::new(UserServiceImpl::default())
}

#[derive(Debug, Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_me(&self, _request: Request<common::Empty>) -> Result<Response<pb::User>, Status> {
        let reply = pb::User {
            id: Uuid::new_v4().to_string(),
            name: format!("Hello {}!", "World").into(),
        };
        Ok(Response::new(reply))
    }

    async fn list(
        &self,
        _request: Request<common::Empty>,
    ) -> Result<Response<pb::UserList>, Status> {
        let reply = pb::User {
            id: Uuid::new_v4().to_string(),
            name: format!("Hello {}!", "World").into(),
        };
        Ok(Response::new(pb::UserList { items: vec![reply] }))
    }
}
