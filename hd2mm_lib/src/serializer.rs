use serde_json::Value;
use crate::error::JsonError;
use crate::manifests::mod_manifest::ModManifest;

pub(crate) trait JsonSerializer<T> {
	fn serialize(&self, value: &T) -> Value;

	fn deserialize(&self, value: &Value) -> Result<T, JsonError>;
}

pub(crate) struct ModManifestSerializerLegacy;

impl JsonSerializer<ModManifest> for ModManifestSerializerLegacy {
	fn serialize(&self, value: &ModManifest) -> Value {
		todo!()
	}

	fn deserialize(&self, value: &Value) -> Result<ModManifest, JsonError> {
		todo!()
	}
}

pub(crate) struct ModManifestSerializerV1;

impl JsonSerializer<ModManifest> for ModManifestSerializerV1 {
	fn serialize(&self, value: &ModManifest) -> Value {
		todo!()
	}

	fn deserialize(&self, value: &Value) -> Result<ModManifest, JsonError> {
		todo!()
	}
}