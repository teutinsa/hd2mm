use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ModError {
	IOError(std::io::Error),
	SerdeError(serde_json::Error)
}

impl Error for ModError { }

impl Display for ModError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ModError::IOError(e) => write!(f, "mod io error \"{}\"", e),
			ModError::SerdeError(e) => write!(f, "mod serde error \"{}\"", e)
		}
	}
}