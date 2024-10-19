use std::path::PathBuf;

use serde_json::{
	json,
	Value
};

use crate::{
	error::JsonError,
	serializer::JsonSerializable
};

#[derive(PartialEq)]
pub struct ProfileManifest {
	name: String,
	path: PathBuf
}

impl ProfileManifest {
	pub fn new(name: String, path: PathBuf) -> Self {
		Self {
			name,
			path
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn path(&self) -> &PathBuf {
		&self.path
	}
}

impl JsonSerializable for ProfileManifest {
	fn serialize(&self) -> Value {
		json!({
			"Name": self.name,
			"Path": self.path
		})
	}

	fn deserialize(value: &Value) -> Result<Self, JsonError> {
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let name = root.get("Name").and_then(Value::as_str).map(|s| s.to_owned()).ok_or(JsonError::ExpectedString("Name".to_owned()))?;
		let path = root.get("Path").and_then(Value::as_str).map(|s| PathBuf::from(s)).ok_or(JsonError::ExpectedString("Name".to_owned()))?;
		Ok(Self {
			name,
			path
		})
	}
}