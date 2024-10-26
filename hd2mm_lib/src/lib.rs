pub(crate) mod manifests;
pub mod errors;

use std::{
	fs,
	path::{
		Path,
		PathBuf
	}
};
use errors::{
	ModError,
	ModManagerError
};
use manifests::{
	mod_manifest::ModManifest,
	profile_manifest::ProfileManifest
};
use log::{
	trace,
	debug,
	info,
	warn,
	error
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
	profiles: Vec<Profile>,
	game_path: PathBuf,
	storage_path: PathBuf,
	temp_path: PathBuf
}

impl ModManager {
	pub fn new(game_path: &PathBuf, storage_path: &PathBuf, temp_path: &PathBuf) -> Result<Self, ModManagerError> {
		Ok(Self {
			profiles: vec![],
			game_path: Self::validate_game_path(game_path).ok_or(ModManagerError::InvalidGamePath)?,
			storage_path: storage_path.to_path_buf(),
			temp_path: temp_path.to_path_buf()
		})
	}

	fn validate_game_path(game_path: &PathBuf) -> Option<PathBuf> {
		let path;
		if game_path.ends_with("Helldivers 2") {
			path = game_path.as_path();
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

// #[macro_export]
// macro_rules! trace_fn {
// 	($struct_name:ident, $fn_name:ident) => {
// 		log::trace!(concat!(stringify!($struct_name), "::", stringify!($fn_name), "()"));
// 	};
// 	($struct_name:ident, $fn_name:ident, $($arg_name:ident),*) => {
// 		log::trace!(
// 			concat!(
// 				stringify!($struct_name), "::", stringify!($fn_name), "(",
// 				$(
// 					stringify!($arg_name), ": {:?}, "
// 				),*,
// 				")"
// 			),
// 			$($arg_name),*
// 		);
// 	};
// }

