use std::{
	fs,
	path::{
		Path,
		PathBuf
	}
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

impl JsonSerializable for ModSubOption {
	fn serialize(&self) -> Value {
		json!({
			"Name": self.name,
			"Description": self.description,
			"Path": self.path
		})
	}

	fn deserialize(value: &Value) -> Result<Self, JsonError> where Self: Sized {
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let get_str = |key: &str| root.get(key).and_then(Value::as_str).ok_or(JsonError::ExpectedString(key.to_owned()));
		let name = get_str("Name")?;
		let description = get_str("Description")?;
		let path = PathBuf::from(get_str("Path")?);
		Ok(Self::new(name.to_owned(), description.to_owned(), path))
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

impl JsonSerializable for ModOption {
	fn serialize(&self) -> Value {
		let opts = self.sub_options.as_deref().map(|s| {
			let mut v = Vec::<Value>::new();
			for o in s {
				v.push(o.serialize());
			}
			v
		});
		json!({
			"Name": self.name,
			"Description": self.description,
			"Path": self.path,
			"Include": self.include,
			"SubOptions": opts
		})
	}

	fn deserialize(value: &Value) -> Result<Self, JsonError> where Self: Sized {
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let get_str = |key: &str| root.get(key).and_then(Value::as_str).ok_or(JsonError::ExpectedString(key.to_owned()));
		let name = get_str("Name")?;
		let description = get_str("Description")?;
		let path = PathBuf::from(get_str("Path")?);
		let mut include: Option<Vec<PathBuf>> = None;
		let mut sub_options: Option<Vec<ModSubOption>> = None;

		if let Some(arr) = root.get("Include").and_then(Value::as_array) {
			if !arr.iter().all(Value::is_string) {
				return Err(JsonError::ExpectedArrayOfString("Include".to_owned()));
			}
			include = Some(arr.iter().map(|v| PathBuf::from(v.as_str().unwrap())).collect());
		}

		if let Some(arr) = root.get("SubOptions").and_then(Value::as_array) {
			if !arr.iter().all(Value::is_object) {
				return Err(JsonError::ExpectedArrayOfObject("SubOptions".to_owned()));
			}
			let mut vec = Vec::<ModSubOption>::new();
			for v in arr {
				vec.push(ModSubOption::deserialize(v)?);
			}
			sub_options = Some(vec);
		}

		Ok(Self::new(name.to_owned(), description.to_owned(), path, include, sub_options))
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
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let id = root.get("Id").and_then(Value::as_i64).ok_or(JsonError::ExpectedString("Id".to_owned()))?;
		let version = root.get("Version").and_then(Value::as_str).ok_or(JsonError::ExpectedString("Version".to_owned()))?;
		Ok(Self::new(id, version.to_owned()))
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
		let root = value.as_object().ok_or(ModManifestError::JsonError(JsonError::ExpectedRootObject))?;
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
	}

	pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ModManifestError> {
		let serializer = ModManifestSerializerV1;
		let value = serializer.serialize(&self);
		let file = fs::File::create(path).map_err(|e| ModManifestError::IOError(e))?;
		serde_json::to_writer(file, &value).map_err(|e| ModManifestError::SerdeError(e))
	} 
}

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn nexus_data_serialize() {
		let dat = ModNexusData::new(105, "1.0.1".to_owned());
		println!("{:?}", dat.serialize());
	}

	#[test]
	fn mod_option_serialize() {
		let sub = ModSubOption::new("Test".to_owned(), "An selectable sub option.".to_owned(), PathBuf::from("opt"));
		let opt = ModOption::new("Test".to_owned(), "An enablable option.".to_owned(), PathBuf::from("opt"), Some(vec![ PathBuf::from("def") ]), Some(vec![ sub ]));
		println!("{:?}", opt.serialize());
	}

	#[test]
	fn sub_option_serialize() {
		let sub = ModSubOption::new("Test".to_owned(), "An selectable sub option.".to_owned(), PathBuf::from("opt"));
		println!("{:?}", sub.serialize());
	}
}