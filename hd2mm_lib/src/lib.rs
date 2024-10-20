pub(crate) mod manifests;
pub mod errors;

use std::{
	fs,
	path::{
		Path,
		PathBuf
	}
};
use errors::ModError;
use manifests::{
	mod_manifest::ModManifest,
	profile_manifest::ProfileManifest
};

pub struct Mod {
	manifest: ModManifest,
	path: PathBuf
}

impl Mod {
	fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ModError> {
		let file = fs::File::open(path.as_ref()).map_err(|e| ModError::IOError(e))?;
		let manifest: ModManifest = serde_json::from_reader(file).map_err(|e| ModError::SerdeError(e))?;
		Ok(Self {
			manifest,
			path: path.as_ref().to_path_buf()
		})
	}

	fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), ModError> {
		let file = fs::File::create(path.as_ref()).map_err(|e| ModError::IOError(e))?;
		serde_json::to_writer_pretty(file, &self.manifest).map_err(|e| ModError::SerdeError(e))?;
		Ok(())
	}
}

pub struct Profile {
	manifest: ProfileManifest,
	path: PathBuf,
	mods: Vec<Mod>
}

pub struct ModManager {
	profiles: Vec<Profile>
}