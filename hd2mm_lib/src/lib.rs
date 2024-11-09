pub mod errors;
pub mod manifests;

use errors::{
	AddError, DeployError, ModError, ModManagerError
};
use fs_more::directory::DirectoryMoveOptions;
use log::{debug, error, info, warn};
use manifests::{
	mod_manifest::{ModManifest, ModOption, ModSubOption},
	profile_manifest::ProfileManifest
};
use uuid::Uuid;
use std::{
	collections::{HashMap, HashSet}, fs, io, path::{
		self, Path, PathBuf
	}
};
use trace_fn::trace_fn;

#[derive(Debug)]
pub struct Mod {
	manifest: ModManifest,
	path: PathBuf,
}

impl Mod {
	#[trace_fn]
	fn from_file<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Self, ModError> {
		let file = fs::File::open(path.as_ref()).map_err(|e| ModError::IOError(e))?;
		let manifest: ModManifest = serde_json::from_reader(file).map_err(|e| ModError::SerdeError(e))?;
		Ok(Self {
			manifest,
			path: path.as_ref().parent().ok_or(ModError::IOError(io::Error::from(io::ErrorKind::NotFound)))?.to_path_buf(),
		})
	}

	#[trace_fn]
	fn to_file<P: AsRef<Path> + std::fmt::Debug>(&self, path: P) -> Result<(), ModError> {
		let file = fs::File::create(path.as_ref()).map_err(|e| ModError::IOError(e))?;
		serde_json::to_writer_pretty(file, &self.manifest).map_err(|e| ModError::SerdeError(e))?;
		Ok(())
	}

	pub fn manifest(&self) -> &ModManifest {
		&self.manifest
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

#[derive(Debug)]
pub struct Profile {
	manifest: ProfileManifest,
	path: PathBuf,
}

impl Profile {
	pub fn new(manifest: ProfileManifest, path: PathBuf) -> Self {
		Self {
			manifest,
			path
		}
	}

	pub fn manifest_mut(&mut self) -> &mut ProfileManifest {
		&mut self.manifest
	}

	pub fn manifest(&self) -> &ProfileManifest {
		&self.manifest
	}

	pub fn path(&self) -> &Path {
		&self.path
	}
}

#[derive(Debug)]
pub struct ModManager {
	mods: Vec<Mod>,
	profiles: Vec<Profile>,
	game_path: PathBuf,
	storage_path: PathBuf,
	temp_path: PathBuf,
	mod_storage_path: PathBuf,
	profile_storage_path: PathBuf
}

impl ModManager {
	#[trace_fn]
	pub fn new(
		game_path: &Path,
		storage_path: &Path,
		temp_path: &Path,
	) -> Result<Self, ModManagerError> {
		let game_path = Self::validate_game_path(game_path)
			.ok_or(ModManagerError::InvalidGamePath)?;
		let storage_path = Self::validate_storage_path(storage_path)
			.ok_or(ModManagerError::InvalidStoragePath)?;
		let temp_path = Self::validate_temp_path(temp_path)
			.ok_or(ModManagerError::InvalidTempPath)?;
		let mod_storage_path = storage_path.join(
			if cfg!(test) {
				"Mods"
			} else {
				"mods"
			}
		);
		Self::ensure_dir_exists(&mod_storage_path)?;
		let profile_storage_path = storage_path.join("profiles");
		Self::ensure_dir_exists(&profile_storage_path)?;

		let mut mods = Vec::new();
		for entry in fs::read_dir(&mod_storage_path)? {
			let mut entry = entry?.path();
			if entry.is_dir() {
				entry.push("manifest.json");
				let r#mod = Mod::from_file(&entry).map_err(|e| ModManagerError::ModError(entry, e))?;
				mods.push(r#mod);
			}
		}
		
		Ok(Self {
			mods,
			profiles: vec![],
			game_path,
			storage_path,
			temp_path,
			mod_storage_path,
			profile_storage_path
		})
	}

	#[trace_fn]
	fn validate_game_path(game_path: &Path) -> Option<PathBuf> {
		let path;
		if game_path.ends_with("Helldivers 2") {
			path = game_path;
		} else {
			path = game_path.parent()?;
		}
		if path.ends_with("Helldivers 2") && path.join("data").is_dir() && path.join("bin").is_dir() {
			Some(PathBuf::from(path))
		} else {
			None
		}
	}

	#[trace_fn]
	fn validate_storage_path(storage_path: &Path) -> Option<PathBuf> {
		if storage_path.is_dir() {
			fs::create_dir_all(storage_path).ok()?;
			Some(storage_path.to_path_buf())
		} else {
			None
		}
	}

	#[trace_fn]
	fn validate_temp_path(temp_path: &Path) -> Option<PathBuf> {
		if temp_path.is_dir() {
			fs::create_dir_all(temp_path).ok()?;
			Some(temp_path.to_path_buf())
		} else {
			None
		}
	}

	fn ensure_dir_exists(path: &Path) -> Result<(), io::Error> {
		if !path.exists() {
			fs::create_dir_all(path)?;
		}
		Ok(())
	}

	#[trace_fn]
	pub fn add_mod<P: AsRef<Path> + std::fmt::Debug>(&mut self, archive_file: P) -> Result<Uuid, AddError> {
		info!("Attempting to add mod from archive \"{:?}\"", archive_file);

		// check if given path is a file
		if !archive_file.as_ref().is_file() {
			return Err(AddError::NotAFile);
		}

		// extract archive to temp directory
		let path = tempdir::TempDir::new_in(&self.temp_path, archive_file.as_ref().file_stem().unwrap().to_str().unwrap()).map_err(AddError::IOError)?;
		info!("Making temp directory at \"{:?}\"", path.path());
		info!("Extracting archive");
		{
			let archive = fs::File::open(&archive_file).map_err(AddError::IOError)?;
			zip_extract::extract(archive, &path.path(), true).map_err(AddError::ZipExtractError)?;
		}
		info!("Extraction complete");

		// look for manifest
		info!("Looking for manifest");
		let manifest_path = path.path().join("manifest.json");
		let mod_name: String;
		let mod_guid: Uuid;
		if manifest_path.exists() { // use manifest
			info!("Manifest found");
			let manifest_file = fs::File::open(manifest_path).map_err(AddError::IOError)?;
			info!("Parsing manifest");
			let manifest: ModManifest = serde_json::from_reader(manifest_file).map_err(AddError::SerdeError)?;

			// check if mod already exists
			info!("Checking if mod already exists");
			if self.has_mod(manifest.guid()) {
				return Err(AddError::AlreadyExists(manifest.guid().clone()));
			}

			// store name
			mod_name = manifest.name().to_string();
			mod_guid = manifest.guid().clone();
		} else { // inferr manifest
			warn!("Not manifest found attempting to inferr");

			// count sub dirs and files
			debug!("Counting sub dirs and files");
			let mut dirs = Vec::new();
			let mut file_count = 0;
			for entry in fs::read_dir(path.path()).map_err(AddError::IOError)? {
				let entry = entry.map_err(AddError::IOError)?;
				let entry = entry.path();
				if entry.is_dir() {
					dirs.push(entry);
				} else if entry.is_file() {
					file_count += 1;
				}
			}
			debug!("Found {} files and {} sub dirs", file_count, dirs.len());

			if dirs.len() == 0 && file_count == 0 {
				error!("This mod is empty");
				return Err(AddError::CanNotInferr);
			}

			// inferr name
			mod_name = archive_file.as_ref().file_stem().unwrap().to_str().unwrap().to_string();
			
			// convert sub dirs to sub options
			let opts: Vec<ModSubOption> = dirs.iter().map(|p| {
				let part = p.file_name().unwrap().to_str().unwrap().to_string();
				ModSubOption::new(part.clone(), String::new(), vec![ PathBuf::from(part) ])
			}).collect();

			// make default option with root include
			let opt = ModOption::new("Default".to_string(), String::new(), Some(vec![ PathBuf::from(".") ]), Some(opts));
			
			let manifest = ModManifest::new(self.make_unique_uuid(), mod_name.clone(), "".to_string(), None, Some(vec![ opt ]), None);
			debug!("Generated manifest {:#?}", manifest);
			
			info!("Writing inferred manifest");
			let manifest_file = fs::File::create_new(manifest_path).map_err(AddError::IOError)?;
			serde_json::to_writer(manifest_file, &manifest).map_err(AddError::SerdeError)?;
			info!("Inferring complete");

			mod_guid = manifest.guid().clone();
		}

		let storage_path = self.storage_path.join(mod_name);
		fs_more::directory::move_directory(&path, &storage_path, DirectoryMoveOptions::default()).map_err(AddError::MoveDirError)?;
		
		let r#mod = Mod::from_file(storage_path.join("manifest.json")).map_err(AddError::ModError)?;
		self.mods.push(r#mod);

		Ok(mod_guid)
	}

	#[trace_fn]
	pub fn remove_mod(&mut self, guid: Uuid) -> Result<(), io::Error> {
		if let Some(i) = self.mods.iter().position(|m| m.manifest().guid() == &guid) {
			let r#mod = self.mods.remove(i);
			fs::remove_dir_all(r#mod.path())?;
		}
		Ok(())
	}

	pub fn has_mod(&self, uuid: &Uuid) -> bool {
		self.mods.iter().any(|m| m.manifest.guid() == uuid)
	}

	#[trace_fn]
	pub fn add_profile(&mut self, name: String) {
		let file_name = sanitize_filename::sanitize(format!("{}.json", name));
		let profile_path = self.profile_storage_path.join(file_name);
		self.profiles.push(Profile::new(ProfileManifest::new(name, HashMap::new()), profile_path));
	}

	#[trace_fn]
	pub fn purge(&self) -> Result<(), DeployError> {
		let data_path = self.game_path.join("data");

		let mut files = Vec::new();
		for entry in fs::read_dir(data_path)? {
			let entry = entry?.path();
			if entry.is_file() {
				if Self::get_file_name(&entry)?.contains("patch_") {
					files.push(entry);
				}
			}
		}

		for file in files {
			fs::remove_file(file)?;
		}

		Ok(())
	}

	#[trace_fn]
	pub fn deploy(&self, profile_index: usize) -> Result<(), DeployError> {
		struct FileTriplet {
			toc_file: Option<PathBuf>,
			steam_file: Option<PathBuf>,
			gpu_file: Option<PathBuf>
		}

		#[trace_fn]
		fn get_patch_files(path: &Path) -> Result<Vec<([char; 16], FileTriplet)>, DeployError> {
			debug!("Grouping {:?}", path);
			let mut files = Vec::new();
			let mut names = HashSet::new();
			for entry in fs::read_dir(path)? {
				let entry = entry?.path();
				if entry.is_file() {
					if ModManager::get_file_name(&entry)?.contains(".patch_") {
						files.push(entry);
					}
				}
			}
			for file in &files {
				let name: [char; 16] = ModManager::get_patch_name(&file)?;
				names.insert(name);
			}
			let mut group = Vec::new();
			for name in &names {
				let mut indexes = HashSet::new();
				for file in &files {
					indexes.insert(ModManager::get_patch_index(file)?);
				}
				for index in indexes {
					let toc_file = files.iter()
						.find(|p| p.ends_with(format!("{}.patch_{}", name.iter().collect::<String>(), index)))
						.map(|p| p.clone());
					let steam_file = files.iter()
						.find(|p| p.ends_with(format!("{}.patch_{}.stream", name.iter().collect::<String>(), index)))
						.map(|p| p.clone());
					let gpu_file = files.iter()
						.find(|p| p.ends_with(format!("{}.patch_{}.gpu_resources", name.iter().collect::<String>(), index)))
						.map(|p| p.clone());
					group.push(
						(
							name.clone(),
							FileTriplet {
								toc_file,
								steam_file,
								gpu_file
							}
						)
					);
				}
			}
			Ok(group)
		}

		if !cfg!(test) {
			self.purge()?;
		}

		let profile = self.profiles.get(profile_index).ok_or(DeployError::ProfileNotFound(profile_index))?;
		info!("Deploying profile \"{}\"", profile.manifest().name());

		let mut mods = Vec::new();
		for key in profile.manifest().mods() {
			if let Some(r#mod) = self.get_mod(key) {
				mods.push(r#mod);
			}
		}
		debug!("Found {} mods", mods.len());

		if mods.is_empty() {
			return Ok(());
		}
		
		debug!("Grouping files");
		let mut groups = HashMap::<[char; 16], Vec<FileTriplet>>::new();
		for r#mod in mods {
			debug!("Looking for enabled options in \"{}\"", r#mod.manifest().name());
			let manifest = r#mod.manifest();
			if let Some(opts) = manifest.options() {
				for (i, opt) in opts.iter().enumerate() {
					if let Some(state) = profile.manifest().get_state(r#mod.manifest().guid()) {
						if state.options()[i] {
							debug!("Using option \"{}\"", opt.name());
							if let Some(includes) = opt.include() {
								for inc in includes {
									for (name, triplet) in get_patch_files(inc)? {
										if !groups.contains_key(&name) {
											groups.insert(name.clone(), Vec::new());
										}
										groups.get_mut(&name).unwrap().push(triplet);
									}
								}
							}
							if let Some(subs) = opt.sub_options() {
								let sub = &subs[state.sub_options()[i]];
								debug!("Using suboption {}", sub.name());
								for inc in sub.include() {
									for (name, triplet) in get_patch_files(inc)? {
										if !groups.contains_key(&name) {
											groups.insert(name.clone(), Vec::new());
										}
										groups.get_mut(&name).unwrap().push(triplet);
									}
								}
							}
						}
					}
				}
			} else {
				debug!("No options specefied, using root");
				for (name, triplet) in get_patch_files(r#mod.path())? {
					if !groups.contains_key(&name) {
						groups.insert(name.clone(), Vec::new());
					}
					groups.get_mut(&name).unwrap().push(triplet);
				}
			}
		}
		debug!("Grouping done");

		Ok(())
	}

	#[trace_fn]
	pub fn save(&self) -> Result<(), std::io::Error> {
		todo!()
	}

	#[trace_fn]
	pub fn get_mod(&self, guid: &Uuid) -> Option<&Mod> {
		self.mods.iter().find(|m| m.manifest().guid() == guid)
	}

	pub fn mods(&self) -> &[Mod] {
		&self.mods
	}

	pub fn profiles_mut(&mut self) -> &mut [Profile] {
		&mut self.profiles
	}

	pub fn profiles(&self) -> &[Profile] {
		&self.profiles
	}

	fn make_unique_uuid(&self) -> Uuid {
		let mut uuid;
		loop {
			uuid = Uuid::new_v4();
			if !self.has_mod(&uuid) {
				break;
			}
		}
		uuid
	}

	#[trace_fn]
	fn get_file_name(path: &Path) -> Result<&str, DeployError> {
		Ok(
			path.file_name()
				.ok_or(DeployError::NameExtractError)?
				.to_str()
				.ok_or(DeployError::NameExtractError)?
		)
	}

	#[trace_fn]
	fn get_patch_name(path: &Path) -> Result<[char; 16], DeployError> {
		Ok(
			path.file_name()
				.ok_or(DeployError::NameExtractError)?
				.to_str()
				.ok_or(DeployError::NameExtractError)?
				.chars()
				.take(16)
				.collect::<Vec<char>>()
				.try_into()
				.map_err(|_| DeployError::NameExtractError)?
		)
	}

	#[trace_fn]
	fn get_patch_index(path: &Path) -> Result<u32, DeployError> {
		Ok(
			path.file_name()
				.ok_or(DeployError::NameExtractError)?
				.to_str()
				.ok_or(DeployError::NameExtractError)?
				.chars()
				.skip(16 + ".patch_".len())
				.take_while(|c| c.is_ascii_digit())
				.collect::<String>()
				.parse::<u32>()?
		)
	}
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use uuid::Uuid;

    use crate::{manifests::profile_manifest::ModState, ModManager};

	#[test]
	fn test_index_parsing() {
		let index = "9ba626afa44a3aa3.patch_12".chars()
			.skip(16 + ".patch_".len())
			.take_while(|c| c.is_ascii_digit())
			.collect::<String>()
			.parse::<u32>()
			.unwrap();
		assert_eq!(index, 12);
	}

	#[test]
	fn test_deploy() {
		simple_logger::SimpleLogger::new()
			.with_colors(true)
			.with_level(log::LevelFilter::Trace)
			.init()
			.unwrap();

		let mut manager = ModManager::new(
			Path::new("D:/SteamLibrary/steamapps/common/Helldivers 2"),
			Path::new("C:/Users/FloCo/AppData/Local/Helldivers2ModManager"),
			Path::new("C:/Users/FloCo/AppData/Local/Temp/Helldivers2ModManager")
		).unwrap();

		manager.add_profile("Default".to_string());

		let states = manager.mods().iter().map(|m| {
			let manifest = m.manifest();
			let options = Vec::with_capacity(
				manifest.options()
					.map_or(0, |opts| opts.len())
			);
			let sub_options = Vec::with_capacity(
				manifest.options()
					.map_or(0, |opts| opts.len())
			);
			(
				manifest.guid().clone(),
				ModState::new(true, options, sub_options)
			)
		}).collect::<Vec<_>>();

		for (guid, state) in states {
			manager.profiles_mut()[0]
				.manifest_mut()
				.add_mod(&guid, state);
		}

		manager.deploy(0).unwrap();
	}
}