// this will have the error handling logic
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    InputOpen(#[from] std::io::Error),
    #[error("IO Error: {0}")]
    OutputWrite(#[from] std::io::Error),
    #[error("CSV Error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type AppResult<T> = Result<T, AppError>;