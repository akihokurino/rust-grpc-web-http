pub mod prefecture;

use crate::errors::{AppError, AppResult};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Method, Response, Url};

#[derive(Clone, Debug)]
pub struct Client {
    base_url: Url,
}

impl Client {
    pub fn new(url: String) -> Self {
        Client {
            base_url: url.parse().unwrap(),
        }
    }

    async fn call(&self, input: CallInput, token: String) -> AppResult<Response> {
        let mut url = self.base_url.clone();
        url.set_path(format!("{}", input.path).as_str());
        for q in input.query {
            url.query_pairs_mut()
                .append_pair(q.0.as_str(), q.1.as_str());
        }
        println!("call api: {}", url.to_string());

        let mut req = reqwest::Request::new(input.method, url);

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        if !token.is_empty() {
            headers.insert(
                "X-Debug-User-Id",
                HeaderValue::from_str(&format!("{}", token)).unwrap(),
            );
        }
        *req.headers_mut() = headers;

        *req.body_mut() = input.body;

        let cli = reqwest::Client::new();
        let resp = cli.execute(req).await.map_err(|e| -> AppError {
            println!("error: {}", e.to_string());
            AppError::from(e)
        })?;

        Ok(resp)
    }
}

#[derive(Default)]
pub struct CallInput {
    pub method: Method,
    pub path: String,
    pub body: Option<Body>,
    pub query: Vec<(String, String)>,
}
