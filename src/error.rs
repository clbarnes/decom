use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    General(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown compression format with magic bytes: {magic_bytes:x?}")]
    UnsupportedFormat { magic_bytes: Vec<u8> },
}

impl Error {
    #[allow(dead_code)]
    pub(crate) fn general<S: Into<String>>(message: S) -> Self {
        Error::General(message.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
