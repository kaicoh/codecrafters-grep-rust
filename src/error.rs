use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("io -- {0}")]
    Io(#[from] std::io::Error),

    #[error("other -- {0}")]
    Other(#[from] anyhow::Error),
}
