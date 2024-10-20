use serde::{
	Deserialize,
	Serialize
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProfileManifest {
	name: String,
	description: String
}