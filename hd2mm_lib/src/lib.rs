pub mod error;
pub mod manifests;
pub(crate) mod serializer;

use std::path::{
	Path,
	PathBuf
};
use error::ModManagerError;
use manifests::{
	mod_manifests::ModManifest,
	profile_manifest::ProfileManifest
};

pub struct Mod {
	manifest: ModManifest,
	path: PathBuf,
}

impl Mod {
	pub fn new(manifest: ModManifest, path: PathBuf) -> Self {
		Self {
			manifest,
			path
		}
	}

	pub fn manifest(&self) -> &ModManifest {
		&self.manifest
	}

	pub fn path(&self) -> &PathBuf {
		&self.path
	}
}

pub struct Profile {
	manifest: ProfileManifest,
	mods: Vec<Mod>
}

impl Profile {
	pub fn new(manifest: ProfileManifest, mods: Vec<Mod>) -> Self {
		Self {
			manifest,
			mods
		}
	}

	pub fn manifest(&self) -> &ProfileManifest {
		&self.manifest
	}

	pub fn mods(&self) -> &[Mod] {
		&self.mods
	}
}

pub struct ModManager {
	storage_path: PathBuf,
	temp_path: PathBuf,
	game_path: PathBuf,
	profiles: Vec<Profile>
}

impl ModManager {
	pub fn new(storage_path: &Path, temp_path: &Path, game_path: &Path) -> Result<Self, ModManagerError> {
		Ok(Self {
			storage_path: storage_path.to_path_buf(),
			temp_path: temp_path.to_path_buf(),
			game_path: Self::validate_game_path(game_path).ok_or(ModManagerError::InvalidGamePath)?,
			profiles: vec![]
		})
	}

	pub fn profiles(&self) -> &[Profile] {
		&self.profiles
	}

	pub fn add_profile(&mut self, profile: Profile) -> Result<(), ModManagerError> {
		todo!()
	}

	pub fn remove_profile(&mut self, profile_index: isize) {
		todo!()
	}

	pub fn add_mod(&mut self, profile_index: isize) -> Result<(), ModManagerError> {
		todo!()
	}

	pub fn remove_mod(&mut self, profile_index: isize, mod_index: isize) {
		todo!()
	}

	pub fn save(&self) -> std::io::Result<()> {
		todo!()
	}

	pub fn deploy(&self, profile_index: isize) -> Result<(), ModManagerError> {
		todo!()
	}

	pub fn purge(&self) -> std::io::Result<()> {
		todo!()
	}

	fn validate_game_path(game_path: &Path) -> Option<PathBuf> {
		let path;
		if game_path.ends_with("Helldivers 2") {
			path = Some(game_path)?;
		} else {
			path = game_path.parent()?;
		}
		if path.ends_with("Helldivers 2") && path.join("data").is_dir() && path.join("bin").is_dir() {
			Some(PathBuf::from(path))
		} else {
			None
		}
	}
}