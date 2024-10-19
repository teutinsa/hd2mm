use std::path::PathBuf;

use serde_json::{
	json,
	Value
};
use uuid::Uuid;
use crate::error::JsonError;
use crate::manifests::mod_manifests::{
	ModManifest,
	ModNexusData,
	ModOption, ModSubOption
};

pub(crate) trait JsonSerializable {
	fn serialize(&self) -> Value;

	fn deserialize(value: &Value) -> Result<Self, JsonError> where Self: Sized;
}

pub(crate) trait JsonSerializer<T> {
	fn serialize(&self, value: &T) -> Value;

	fn deserialize(&self, value: &Value) -> Result<T, JsonError>;
}

pub(crate) struct ModManifestSerializerLegacy;

impl JsonSerializer<ModManifest> for ModManifestSerializerLegacy {
	fn serialize(&self, _value: &ModManifest) -> Value {
		panic!("this is legacy and will never be implemented")
	}

	fn deserialize(&self, value: &Value) -> Result<ModManifest, JsonError> {
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let get_str = |key: &str| root.get(key).and_then(Value::as_str).ok_or(JsonError::ExpectedString(key.to_owned()));
		let guid = Uuid::parse_str(get_str("Guid")?).map_err(|_| JsonError::ExpectedArrayOfString("Guid".to_owned()))?;
		let name = get_str("Name")?;
		let description = get_str("Description")?;
		let icon_path = root.get("IconPath").and_then(Value::as_str).map(|s| PathBuf::from(s));

		let mut subs: Option<Vec<ModSubOption>> = None;
		if let Some(opts) = root.get("Options").and_then(Value::as_array) {
			if !opts.iter().all(Value::is_string) {
				return Err(JsonError::ExpectedArrayOfString("Options".to_owned()));
			}

			subs = Some(opts.iter().map(|s| {
				let str = s.as_str().unwrap();
				ModSubOption::new(str.to_owned(), String::new(), PathBuf::from(str))
			}).collect());
		}

		let opts = subs.map(|v| vec![ ModOption::new("Main".to_owned(), String::new(), vec![ PathBuf::from(".") ], Some(v)) ]);
		Ok(ModManifest::new(guid, name.to_owned(), description.to_owned(), icon_path, opts, None))
	}
}

pub(crate) struct ModManifestSerializerV1;

impl JsonSerializer<ModManifest> for ModManifestSerializerV1 {
	fn serialize(&self, value: &ModManifest) -> Value {
		let options: Option<Vec<Value>> = value.options().map(|s| s.iter().map(|o| o.serialize()).collect());
		let data = value.nexus_data().map(|d| d.serialize());
		json!({
			"Version": value.version(),
			"Guid": value.guid().to_string(),
			"Name": value.name(),
			"Description": value.description(),
			"IconPath": value.icon_path(),
			"Options": options,
			"NexusData": data
		})
	}

	fn deserialize(&self, value: &Value) -> Result<ModManifest, JsonError> {
		let root = value.as_object().ok_or(JsonError::ExpectedRootObject)?;
		let get_str = |key: &str| root.get(key).and_then(Value::as_str).ok_or(JsonError::ExpectedString(key.to_owned()));
		let guid = Uuid::parse_str(get_str("Guid")?).map_err(|_| JsonError::ExpectedArrayOfString("Guid".to_owned()))?;
		let name = get_str("Name")?;
		let description = get_str("Description")?;
		let icon_path = root.get("IconPath").and_then(Value::as_str).map(|s| PathBuf::from(s));
		let mut options = None;
		let mut nexus_data = None;

		if let Some(arr) = root.get("Options").and_then(Value::as_array) {
			if !arr.iter().all(Value::is_object) {
				return Err(JsonError::ExpectedArrayOfObject("Options".to_owned()));
			}

			let mut vec = Vec::new();
			for v in arr {
				vec.push(ModOption::deserialize(v)?);
			}
			options = Some(vec);
		}

		if let Some(obj) = root.get("NexusData").and_then(Value::as_object) {
			nexus_data = Some(ModNexusData::deserialize(&Value::Object(obj.to_owned()))?);
		}

		Ok(ModManifest::new(guid, name.to_owned(), description.to_owned(), icon_path, options, nexus_data))
	}
}