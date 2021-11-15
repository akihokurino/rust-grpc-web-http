use tonic::{Request, Response, Status};

use crate::external::prefecture::get_prefectures::Input;
use crate::external::Client;
use pb::prefecture_service_server::{PrefectureService, PrefectureServiceServer};

pub mod common {
    tonic::include_proto!("common");
}
pub mod pb {
    tonic::include_proto!("prefecture");
}

pub fn new_server(cli: Client) -> PrefectureServiceServer<PrefectureServiceImpl> {
    PrefectureServiceServer::new(PrefectureServiceImpl { cli })
}

#[derive(Debug)]
pub struct PrefectureServiceImpl {
    cli: Client,
}

#[tonic::async_trait]
impl PrefectureService for PrefectureServiceImpl {
    async fn list(
        &self,
        _request: Request<common::Empty>,
    ) -> Result<Response<pb::PrefectureList>, Status> {
        let prefs = self.cli.get_list(Input {}).await.map_err(Status::from)?;

        Ok(Response::new(pb::PrefectureList {
            items: prefs
                .items
                .iter()
                .map(|v| pb::Prefecture {
                    id: v.code.to_owned().unwrap(),
                    name: v.name.to_owned().unwrap(),
                })
                .collect(),
        }))
    }
}
