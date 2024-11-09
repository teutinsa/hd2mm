use std::path::PathBuf;

use fs_more::error::MoveDirectoryError;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ModError {
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("serde error")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum ModManagerError {
    #[error("invalid game path")]
    InvalidGamePath,
    #[error("invalid temporary path")]
    InvalidTempPath,
    #[error("invalid storage path")]
    InvalidStoragePath,
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("error in mod \"{0}\"")]
    ModError(PathBuf, #[source] ModError)
}


#[derive(Debug, Error)]
pub enum AddError {
    #[error("given path is not a file")]
    NotAFile,
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("zip extract error")]
    ZipExtractError(#[from] zip_extract::ZipExtractError),
    #[error("serde error")]
    SerdeError(#[from] serde_json::Error),
    #[error("mod with GUID `{0}` already exists")]
    AlreadyExists(Uuid),
    #[error("move directory error")]
    MoveDirError(#[from] MoveDirectoryError),
    #[error("can not inferr manifest")]
    CanNotInferr,
    #[error("mod error")]
    ModError(#[from] ModError)
}

#[derive(Debug, Error)]
pub enum DeployError {
    #[error("io error")]
    IOError(#[from] std::io::Error),
    #[error("profile {0} not found")]
    ProfileNotFound(usize),
    #[error("error extracting toc name")]
    NameExtractError,
    #[error("")]
    ParseIndexError(#[from] std::num::ParseIntError)
}