use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<quick_xml::DeError> for AppError {
    fn from(e: quick_xml::DeError) -> Self {
        AppError::Internal(e.to_string())
    }
}

impl From<AppError> for worker::Error {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Internal(e) => worker::Error::Internal(e.into()),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
