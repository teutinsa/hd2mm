use std::{
	fs,
	path::Path
};
use serde_json::Value;
use crate::{
	error::{
		JsonError,
		ModManifestError
	},
	serializer::*
};

pub struct ModManifest {

}

impl ModManifest {
	pub fn new() -> Self {
		Self {

		}
	}

	pub fn from_file<P: AsRef<Path>>(&self, path: P) -> Result<Self, ModManifestError> {
		let file = fs::File::open(path).map_err(|e| ModManifestError::IOError(e))?;
		let value = serde_json::from_reader(file).map_err(|e| ModManifestError::SerdeError(e))?;
		match &value {
			Value::Object(root) => {
				let deserializer: Box<dyn JsonSerializer<ModManifest>>;

				if let Some((prop, val)) = root.get_key_value("Version") {
					if let Value::Number(num) = val {
						match num.as_i64() {
							Some(1) => deserializer = Box::new(ModManifestSerializerV1),
							Some(num) => return  Err(ModManifestError::UnknownVersion(num)),
							_ => return Err(ModManifestError::JsonError(JsonError::ExpectedNumber(prop.to_owned())))
						}
					} else {
						return Err(ModManifestError::JsonError(JsonError::ExpectedNumber(prop.to_owned())));
					}
				} else {
					deserializer = Box::new(ModManifestSerializerLegacy);
				}

				deserializer.deserialize(&value).map_err(|e| ModManifestError::JsonError(e))
			},
			_ => Err(ModManifestError::JsonError(JsonError::ExpectedRootObject))
		}
	}

	pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ModManifestError> {
		let serializer = ModManifestSerializerV1;
		let value = serializer.serialize(&self);
		let file = fs::File::create(path).map_err(|e| ModManifestError::IOError(e))?;
		serde_json::to_writer(file, &value).map_err(|e| ModManifestError::SerdeError(e))
	} 
}