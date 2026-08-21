use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid args: {0}")]
    InvalidArgs(String),

    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("skill not found: {0}")]
    SkillNotFound(String),

    #[error("command timed out")]
    Timeout,

    #[error("empty content")]
    EmptyContent,
}
