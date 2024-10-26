use serde::{
	Deserialize,
	Serialize
};
use trace_fn::trace_fn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProfileManifest {
	name: String,
	description: String
}

impl ProfileManifest {
	#[trace_fn]
	pub fn new(name: String, description: String) -> Self {
		Self {
			name,
			description
		}
	}
}