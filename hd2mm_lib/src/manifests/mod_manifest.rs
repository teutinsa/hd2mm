use std::{
	fs,
	path::{Path, PathBuf}
};
use serde_json::{
	json,
	Value
};
use uuid::Uuid;
use crate::{
	error::{
		JsonError,
		ModManifestError
	},
	serializer::*
};

pub struct ModSubOption {
	name: String,
	description: String,
	path: PathBuf
}

impl ModSubOption {
	pub fn new(name: String, description: String, path: PathBuf) -> Self {
		Self {
			name,
			description,
			path
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn description(&self) -> &str {
		&self.description
	}

	pub fn path(&self) -> &PathBuf {
		&self.path
	}
}

pub struct ModOption {
	name: String,
	description: String,
	path: PathBuf,
	include: Option<Vec<PathBuf>>,
	sub_options: Option<Vec<ModSubOption>>
}

impl ModOption {
	pub fn new(name: String, description: String, path: PathBuf, include: Option<Vec<PathBuf>>, sub_options: Option<Vec<ModSubOption>>) -> Self {
		Self {
			name,
			description,
			path,
			include,
			sub_options
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn description(&self) -> &str {
		&self.description
	}

	pub fn path(&self) -> &PathBuf {
		&self.path
	}

	pub fn include(&self) -> Option<&[PathBuf]> {
		self.include.as_deref()
	}

	pub fn sub_options(&self) -> Option<&[ModSubOption]> {
		self.sub_options.as_deref()
	}
}

pub struct ModNexusData {
	id: i64,
	version: String
}

impl ModNexusData {
	pub fn new(id: i64, version: String) -> Self {
		Self {
			id,
			version
		}
	}

	pub fn id(&self) -> i64 {
		self.id
	}

	pub fn version(&self) -> &str {
		&self.version
	}
}

impl JsonSerializable for ModNexusData {
	fn serialize(&self) -> Value {
		json!({
			"Id": self.id,
			"Version": self.version
		})
	}

	fn deserialize(value: &Value) -> Result<Self, JsonError> where Self: Sized {
		if let Some(root) = value.as_object() {
			if let Some(id) = root.get("Id").and_then(Value::as_i64) {
				if let Some(version) = root.get("Version").and_then(Value::as_str) {
					Ok(Self::new(id, version.to_owned()))
				} else {
					Err(JsonError::ExpectedString("Version".to_owned()))
				}
			} else {
				Err(JsonError::ExpectedString("Id".to_owned()))
			}
		} else {
			Err(JsonError::ExpectedRootObject)
		}
	}
}

pub struct ModManifest {
	version: i64,
	guid: Uuid,
	name: String,
	description: String,
	icon_path: Option<PathBuf>,
	options: Option<Vec<ModOption>>,
	nexus_data: Option<ModNexusData>
}

impl ModManifest {
	pub fn new(guid: Uuid, name: String, description: String, icon_path: Option<PathBuf>, options: Option<Vec<ModOption>>, nexus_data: Option<ModNexusData>) -> Self {
		Self {
			version: 1,
			guid,
			name,
			description,
			icon_path,
			options,
			nexus_data
		}
	}

	pub fn version(&self) -> i64 {
		self.version
	}

	pub fn guid(&self) -> Uuid {
		self.guid
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn description(&self) -> &str {
		&self.description
	}

	pub fn icon_path(&self) -> Option<&PathBuf> {
		self.icon_path.as_ref()
	}

	pub fn options(&self) -> Option<&[ModOption]> {
		self.options.as_deref()
	}

	pub fn nexus_data(&self) -> Option<&ModNexusData> {
		self.nexus_data.as_ref()
	}

	pub fn from_file<P: AsRef<Path>>(&self, path: P) -> Result<Self, ModManifestError> {
		let file = fs::File::open(path).map_err(|e| ModManifestError::IOError(e))?;
		let value: Value = serde_json::from_reader(file).map_err(|e| ModManifestError::SerdeError(e))?;
		if let Some(root) = value.as_object() {
			let deserializer: Box<dyn JsonSerializer<ModManifest>>;
			if let Some(num) = root.get("Version").and_then(Value::as_i64) {
				match num {
					1 => deserializer = Box::new(ModManifestSerializerV1),
					num => return Err(ModManifestError::UnknownVersion(num))
				}
			} else {
				deserializer = Box::new(ModManifestSerializerLegacy);
			}
			deserializer.deserialize(&value).map_err(|e| ModManifestError::JsonError(e))
		} else {
			Err(ModManifestError::JsonError(JsonError::ExpectedRootObject))
		}
	}

	pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ModManifestError> {
		let serializer = ModManifestSerializerV1;
		let value = serializer.serialize(&self);
		let file = fs::File::create(path).map_err(|e| ModManifestError::IOError(e))?;
		serde_json::to_writer(file, &value).map_err(|e| ModManifestError::SerdeError(e))
	} 
}