#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to initialize {component}: {reason}")]
    InitError {
        component: String,
        reason: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("freetype error: {0}")]
    FtError(freetype::Error),

    #[error("unknown error: {0}")]
    UnknownError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
