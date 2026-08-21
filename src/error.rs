use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("config error: {0}")]
    Config(String),

    #[error("provider error: {0}")]
    Provider(String),

    #[error("agent error: {0}")]
    Agent(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Agent(e.to_string())
    }
}
