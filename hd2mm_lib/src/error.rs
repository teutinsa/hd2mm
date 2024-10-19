use std::{
	error::Error,
	fmt::{
		Display,
		Formatter
	}
};

#[derive(Debug)]
pub enum JsonError {
	ExpectedRootObject,
	ExpectedRootArray,
	ExpectedObject(String),
	ExpectedArray(String),
	ExpectedBool(String),
	ExpectedNumber(String),
	ExpectedString(String),
	ExpectedObjectOrNull(String),
	ExpectedArrayOrNull(String),
	ExpectedBoolOrNull(String),
	ExpectedNumberOrNull(String),
	ExpectedStringOrNull(String),
	ExpectedArrayOfBool(String),
	ExpectedArrayOfNumber(String),
	ExpectedArrayOfString(String),
	ExpectedArrayOfObject(String),
	ExpectedArrayOfArray(String)
}

impl Display for JsonError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			JsonError::ExpectedRootObject => write!(f, "Expected json root to be an object!"),
			JsonError::ExpectedRootArray => write!(f, "Expected json root to be an array!"),
			JsonError::ExpectedObject(prop) => write!(f, "Property \"{prop}\" to be a object!"),
			JsonError::ExpectedArray(prop) => write!(f, "Property \"{prop}\" to be a array!"),
			JsonError::ExpectedBool(prop) => write!(f, "Property \"{prop}\" to be a boolean!"),
			JsonError::ExpectedNumber(prop) => write!(f, "Property \"{prop}\" to be a number!"),
			JsonError::ExpectedString(prop) => write!(f, "Property \"{prop}\" to be a string!"),
			JsonError::ExpectedObjectOrNull(prop) => write!(f, "Property \"{prop}\" to be a object or null!"),
			JsonError::ExpectedArrayOrNull(prop) => write!(f, "Property \"{prop}\" to be a array or null!"),
			JsonError::ExpectedBoolOrNull(prop) => write!(f, "Property \"{prop}\" to be a boolean or null!"),
			JsonError::ExpectedNumberOrNull(prop) => write!(f, "Property \"{prop}\" to be a number or null!"),
			JsonError::ExpectedStringOrNull(prop) => write!(f, "Property \"{prop}\" to be a string or null!"),
			JsonError::ExpectedArrayOfBool(prop) => write!(f, "Property \"{prop}\" to be an array of booleans!"),
			JsonError::ExpectedArrayOfNumber(prop) => write!(f, "Property \"{prop}\" to be an array of numbers!"),
			JsonError::ExpectedArrayOfString(prop) => write!(f, "Property \"{prop}\" to be an array of strings!"),
			JsonError::ExpectedArrayOfObject(prop) => write!(f, "Property \"{prop}\" to be an array of objects!"),
			JsonError::ExpectedArrayOfArray(prop) => write!(f, "Property \"{prop}\" to be an array of arrays!"),
		}
	}
}

impl Error for JsonError { }

#[derive(Debug)]
pub enum ModManifestError {
	IOError(std::io::Error),
	SerdeError(serde_json::Error),
	JsonError(JsonError),
	UnknownVersion(i64),
}

impl Display for ModManifestError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ModManifestError::IOError(error) => write!(f, "IO error: \"{error}\"!"),
			ModManifestError::SerdeError(error) => write!(f, "Serde error: \"{error}\"!"),
			ModManifestError::JsonError(error) => write!(f, "Json error: \"{error}\"!"),
			ModManifestError::UnknownVersion(num) => write!(f, "Unknown version {num}!"),
		}
	}
}

impl Error for ModManifestError { }

#[derive(Debug)]
pub enum ModManagerError {
	InvalidGamePath,
	IOError(std::io::Error),
	ProfileAlreadyExists
}

impl Display for ModManagerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Error for ModManagerError { }