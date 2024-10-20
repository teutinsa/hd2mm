pub mod manifests;

use std::path::{
	Path,
	PathBuf
};
use manifests::mod_manifest::ModManifest;

pub struct Mod {
	manifest: ModManifest,
	path: PathBuf
}

pub struct Profile {
	path: PathBuf,
	mods: Vec<Mod>
}

pub struct ModManager {
	profiles: Vec<Profile>
}