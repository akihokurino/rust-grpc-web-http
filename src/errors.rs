use reqwest;
use thiserror::Error as ThisErr;
use tonic::Status;

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum AppError {
    #[error("不正なパラメーターです: {0}")]
    BadRequest(String),
    #[error("認証エラーです")]
    UnAuthenticate,
    #[error("禁止された行為です")]
    Forbidden,
    #[error("指定されたリソースが見つかりません")]
    NotFound,
    #[error("サーバーエラーです: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<String> for AppError {
    fn from(v: String) -> Self {
        Self::Internal(v)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        Self::Internal(e.to_string())
    }
}

impl From<AppError> for Status {
    fn from(e: AppError) -> Self {
        match e {
            AppError::BadRequest(_) => Status::invalid_argument(""),
            AppError::UnAuthenticate => Status::unauthenticated(""),
            AppError::Forbidden => Status::permission_denied(""),
            AppError::NotFound => Status::not_found(""),
            AppError::Internal(_) => Status::internal(""),
        }
    }
}
