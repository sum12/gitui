use std::{num::TryFromIntError, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("`{0}`")]
    Generic(String),

    #[error("git: no head found")]
    NoHead,

    #[error("git: remote url not found")]
    UnknownRemote,

    #[error("git: inconclusive remotes")]
    NoDefaultRemoteFound,

    #[error("git: work dir error")]
    NoWorkDir,

    #[error("git: uncommitted changes")]
    UncommittedChanges,

    #[error("git: can\u{2019}t run blame on a binary file")]
    NoBlameOnBinaryFile,

    #[error("io error:{0}")]
    Io(#[from] std::io::Error),

    #[error("git error:{0}")]
    Git(#[from] git2::Error),

    #[error("utf8 error:{0}")]
    Utf8Conversion(#[from] FromUtf8Error),

    #[error("TryFromInt error:{0}")]
    IntConversion(#[from] TryFromIntError),

    #[error("EasyCast error:{0}")]
    EasyCast(#[from] easy_cast::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(error: std::sync::PoisonError<T>) -> Self {
        Self::Generic(format!("poison error: {}", error))
    }
}
