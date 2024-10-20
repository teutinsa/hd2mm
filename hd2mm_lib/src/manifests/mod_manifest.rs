use std::path::PathBuf;
use serde::{
	Deserialize,
	Serialize
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ModSubOption {
	name: String,
	description: String,
	include: Vec<PathBuf>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ModOption {
	name: String,
	description: String,
	include: Option<Vec<PathBuf>>,
	sub_options: Option<Vec<ModSubOption>>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct NexusData {
	id: u32,
	version: String
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub(crate) struct Version<const VER: u32>;

impl<const VER: u32> TryFrom<u32> for Version<VER> {
	type Error = &'static str;

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		if value == VER {
			Ok(Version::<VER>)
		} else {
			Err("version missmatch")
		}
	}
}

impl<const VER: u32> From<Version<VER>> for u32 {
	fn from(_value: Version<VER>) -> Self {
		VER
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ModManifest {
	#[serde(rename_all = "PascalCase")]
	Legacy {
		guid: Uuid,
		name: String,
		description: String,
		icon_path: Option<PathBuf>,
		options: Option<Vec<PathBuf>>
	},
	#[serde(rename_all = "PascalCase")]
	V1 {
		version: Version<1>,
		guid: Uuid,
		name: String,
		description: String,
		icon_path: Option<PathBuf>,
		options: Option<Vec<ModOption>>,
		nexus_data: Option<NexusData>
	}
}

#[cfg(test)]
mod test {
    use super::*;

	#[test]
	fn mod_manifest_deserialize_legacy() {
		let data = r#"
		{
			"Guid": "f9125200-1cc8-484d-acc9-1bfd8fdce4fb",
			"Name": "Jane Helldiver 4K v1.04",
			"Description": "Locally imported mod.",
			"IconPath": null,
			"Options": [
				"Jane Helldiver 4K - Skin B",
				"Jane Helldiver 4K - Skin A"
			]
		}
		"#;
		let value: ModManifest = serde_json::from_str(data).unwrap();
		assert!(matches!(value, ModManifest::Legacy { guid: _, name: _, description: _, icon_path: _, options: _ }))
	}

	#[test]
	fn mod_manifest_deserialize_v1() {
		let data = r#"
		{
			"Version": 1,
			"Guid": "00000000-0000-0000-0000-000000000000",
			"Name": "Test",
			"Description": "A test mod.",
			"Options": [
				{
					"Name": "Default",
					"Description": "The default option.",
					"Include": [
						"(Body)"
					],
					"SubOptions": [
						{
							"Name": "Version A",
							"Description": "Skin A",
							"Include": [
								"Folder A"
							]
						},
						{
							"Name": "Version B",
							"Description": "Skin B",
							"Include": [
								"Folder B"
							]
						}
					]
				}
			]
		}
		"#;
		let value: ModManifest = serde_json::from_str(data).unwrap();
		println!("{:?}", value);
		assert!(matches!(value, ModManifest::V1 { version: _, guid: _, name: _, description: _, icon_path: _, options: _, nexus_data: _ }))
	}

	#[test]
	#[should_panic]
	fn mod_manifest_deserialize_invalid_version() {
		let data = r#"
		{
			"Version": 2,
			"Guid": "00000000-0000-0000-0000-000000000000",
			"Name": "Test",
			"Description": "A test mod.",
			"Options": [
				{
					"Name": "Default",
					"Description": "The default option.",
					"Include": [
						"(Body)"
					],
					"SubOptions": [
						{
							"Name": "Version A",
							"Description": "Skin A",
							"Include": [
								"Folder A"
							]
						},
						{
							"Name": "Version B",
							"Description": "Skin B",
							"Include": [
								"Folder B"
							]
						}
					]
				}
			]
		}
		"#;
		let _: ModManifest = serde_json::from_str(data).unwrap();
	}

	#[test]
	fn mod_manifest_serialize() {
		let manifest = ModManifest::V1 {
			version: Version::<1>,
			guid: Uuid::nil(),
			name: "Test".to_owned(),
			description: "A test mod.".to_owned(),
			icon_path: None,
			options: Some(vec![
				ModOption {
					name: "Default".to_owned(),
					description: "The default option.".to_owned(),
					include: Some(vec![
						PathBuf::from("(Body)")
					]),
					sub_options: Some(vec![
						ModSubOption {
							name: "Version A".to_owned(),
							description: "Skin A".to_owned(),
							include: vec![
								PathBuf::from("Folder A")
							]
						},
						ModSubOption {
							name: "Version B".to_owned(),
							description: "Skin B".to_owned(),
							include: vec![
								PathBuf::from("Folder B")
							]
						}
					])
				}
			]),
			nexus_data: None
		};
		println!("{}", serde_json::to_string(&manifest).unwrap())
	}
}