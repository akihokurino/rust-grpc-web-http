use crate::errors::{AppError, AppResult};
use crate::external::{CallInput, Client};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl Client {
    pub async fn get_list(
        &self,
        _input: get_prefectures::Input,
    ) -> AppResult<get_prefectures::Output> {
        #[derive(Debug, Serialize)]
        struct Body {}

        let body = Body {};

        println!("json body: {}", serde_json::to_string(&body).unwrap());

        let query = vec![];

        self.call(
            CallInput {
                method: Method::POST,
                path: "/twirp/api.PrefectureService/GetAll".to_string(),
                body: Some(
                    serde_json::to_string(&body)
                        .map_err(|e| AppError::Internal(e.to_string()))?
                        .into(),
                ),
                query,
            },
            Uuid::new_v4().to_string(),
        )
        .await?
        .error_for_status()?
        .json::<get_prefectures::Output>()
        .await
        .map_err(AppError::from)
    }
}

pub mod get_prefectures {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Input {}

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub items: Vec<Prefecture>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Prefecture {
        pub code: Option<i32>,
        pub name: Option<String>,
    }
}
