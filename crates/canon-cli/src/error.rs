use thiserror::Error;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Engine(#[from] canon_engine::EngineError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}")]
    Unimplemented(&'static str),
}
