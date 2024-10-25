use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModError {
	#[error("mod io error")]
	IOError(#[from] std::io::Error),
	#[error("mod serde error")]
	SerdeError(#[from] serde_json::Error)
}

#[derive(Debug, Error)]
pub enum ModManagerError {
	#[error("invalid game path")]
	InvalidGamePath,
	#[error("invalid temporary path")]
	InvalidTempPath,
	#[error("invalid storage path")]
	InvalidStoragePath
}