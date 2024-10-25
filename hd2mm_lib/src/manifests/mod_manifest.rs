use std::{
	cell::OnceCell, 
	path::{
		Path,
		PathBuf
	}
};
use serde::{
	Deserialize,
	Serialize
};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModSubOption {
	name: String,
	description: String,
	include: Vec<PathBuf>
}

impl ModSubOption {
	pub fn new(name: String, description: String, include: Vec<PathBuf>) -> Self {
		Self {
			name,
			description,
			include
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn description(&self) -> &str {
		&self.description
	}

	pub fn include(&self) -> &[PathBuf] {
		&self.include
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModOption {
	name: String,
	description: String,
	include: Option<Vec<PathBuf>>,
	sub_options: Option<Vec<ModSubOption>>
}

impl ModOption {
	pub fn new(name: String, description: String, include: Option<Vec<PathBuf>>, sub_options: Option<Vec<ModSubOption>>) -> Self {
		Self {
			name,
			description,
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

	pub fn include(&self) -> Option<&[PathBuf]> {
		self.include.as_deref()
	}

	pub fn sub_options(&self) -> Option<&[ModSubOption]> {
		self.sub_options.as_deref()
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NexusData {
	id: u32,
	version: String
}

impl NexusData {
	pub fn new(id: u32, version: String) -> Self {
		Self {
			id,
			version
		}
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn version(&self) -> &str {
		&self.version
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct Version<const VER: u32>;

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
enum ModData {
	#[serde(rename_all = "PascalCase")]
	Legacy {
		guid: Uuid,
		name: String,
		description: String,
		icon_path: Option<PathBuf>,
		options: Option<Vec<String>>
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ModManifest {
	#[serde(flatten)]
	data: ModData,
	#[serde(skip)]
	transalted_options: OnceCell<[ModOption; 1]>
}

impl ModManifest {
	pub fn new(guid: Uuid, name: String, description: String, icon_path: Option<PathBuf>, options: Option<Vec<ModOption>>, nexus_data: Option<NexusData>) -> Self {
		Self {
			data: ModData::V1 {
				version: Version::<1>,
				guid,
				name,
				description,
				icon_path,
				options,
				nexus_data
			},
			transalted_options: OnceCell::new()
		}
	}

	pub fn guid(&self) -> &Uuid {
		match &self.data {
			ModData::Legacy { guid, .. } => guid,
			ModData::V1 { guid, ..} => guid
		}
	}

	pub fn name(&self) -> &str {
		match &self.data {
			ModData::Legacy { name, .. } => name,
			ModData::V1 { name, .. } => name
		}
	}

	pub fn description(&self) -> &str {
		match &self.data {
			ModData::Legacy { description, .. } => description,
			ModData::V1 { description, .. } => description
		}
	}

	pub fn icon_path(&self) -> Option<&Path> {
		match &self.data {
			ModData::Legacy { icon_path, .. } => icon_path.as_deref(),
			ModData::V1 { icon_path, .. } => icon_path.as_deref()
		}
	}

	pub fn options(&self) -> Option<&[ModOption]> {
		match &self.data {
			ModData::Legacy { options, .. } => {
				options.as_ref().map(|v| {
					self.transalted_options.get_or_init(|| {
						let opts: Vec<ModSubOption> = v.iter().map(|s| ModSubOption::new(s.to_owned(), String::new(), vec![ PathBuf::from(s) ])).collect();
						[ModOption::new("Default".to_owned(), String::new(), None, Some(opts)); 1]
					}).as_slice()
				})
			},
			ModData::V1 { options, .. } => options.as_deref()
		}
	}

	pub fn is_legacy(&self) -> bool {
		match self.data {
			ModData::Legacy { .. } => true,
			_ => false
		}
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
		println!("{:?}", value.options());
		assert!(value.is_legacy())
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
		assert!(!value.is_legacy())
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
		let manifest = ModManifest::new(
			Uuid::nil(),
			"Test".to_owned(),
			"A test mod.".to_owned(),
			None,
			Some(vec![
				ModOption::new(
					"Default".to_owned(),
					"The default option.".to_owned(),
					Some(vec![
						PathBuf::from("(Body)")
					]),
					Some(vec![
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
				)
			]),
			None
		);
		println!("{}", serde_json::to_string(&manifest).unwrap())
	}
}