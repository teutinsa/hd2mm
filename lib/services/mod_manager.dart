import 'dart:collection';
import 'dart:convert';
import 'dart:io';

import 'package:archive/archive_io.dart' show extractFileToDisk;
import 'package:json5/json5.dart';
import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;
import 'package:uuid/uuid.dart';

import '../helpers/directory_extensions.dart';
import '../errors/not_initialized_error.dart';
import '../models/mod.dart';
import '../models/mod_manifest.dart';
import '../models/profile.dart';
import '../models/settings.dart';
import '../models/mod_data.dart';

enum ManagerInitProgress {
  loadingMods,
  loadingProfiles,
  checkingProfiles,
  complete,
}

typedef _PatchFileTriplet = ({File? patch, File? gpuResources, File? stream});

enum _RarHandler {
  unrar,
  $7zip,
}

final class ModManagerService {
  Profile get activeProfile => _profiles.profiles[_profiles.active];

  set activeProfile(Profile value) {
    final i = _profiles.profiles.indexOf(value);
    if (i == -1) return;
    _profiles.active = i;
  }

  Profile? get deployedProfile => _profiles.deployed != null
    ? _profiles.profiles[_profiles.deployed!]
    : null;

  UnmodifiableListView<Mod> get mods => UnmodifiableListView(_mods);

  UnmodifiableListView<Profile> get profiles => UnmodifiableListView(_profiles.profiles);

  List<String> get supportedExtensions => _extensions;

  static final _patchFileRegex = RegExp(r"^[a-z0-9]{16}\.patch_[0-9]+(\.(stream|gpu_resources))?$");
  static final _patchRegex = RegExp(r"\.patch_[0-9]+");
  static final _patchIndexRegex = RegExp(r"^(?:[a-z0-9]{16}\.patch_)([0-9]+)(?:(?:\.(?:stream|gpu_resources))?)$");
  // This Regex was a stupid idea! I hate it! God, why did I decide on using it...
  static final _archiveFileRegex = RegExp(r"^(?<name>[a-zA-Z0-9-]+?)-(?<mod_id>(?!0)\d+)-(?<version>[0-9-]+[a-zA-Z0-9]+?)-(?<file_id>\d+)\.(?<ext>\w+)$");
  final _log = Logger("ModManagerService");
  bool _initialized = false;
  _RarHandler? _rarHandler;
  bool _7zSupported = false;
  late List<String> _extensions;
  late Settings _settings;
  late Directory _modsDir;
  final List<Mod> _mods = [];
  late File _profilesFile;
  late ProfileData _profiles;

  ModManagerService();

  Future<void> init(Settings settings, [void Function(ManagerInitProgress)? progressCallback]) async {
    _mods.clear();

    _log.info("Initializing manager service...");
    _settings = settings;
    
    _log.info("Loading mods...");
    progressCallback?.call(ManagerInitProgress.loadingMods);
    _modsDir = Directory(path.join(_settings.storagePath.path, "mods"));
    if (await _modsDir.exists()) {
      await for (final entry in _modsDir.list()) {
        if (entry is! Directory) continue;
        
        final manifestFile = await entry.tryGetFile("manifest.json");
        
        if (manifestFile == null) {
          _log.warning("Directory \"${path.basename(entry.path)}\" does not contain a manifest. Consider removing it.");
          continue;
        }

        try {
          final source = await manifestFile.readAsString();
          final json = json5Decode(source) as Map<String, dynamic>;
          final manifest = ModManifest.fromJson(json);
          _mods.add(Mod(entry, manifest));
        } on Exception catch (e) {
          _log.severe("Error reading manifest in directory \"${path.basename(entry.path)}\"!", e);
        }
      }
    } else {
      await _modsDir.create(recursive: true);
    }

    _log.info("Loading profiles...");
    progressCallback?.call(ManagerInitProgress.loadingProfiles);
    _profilesFile = File(path.join(_settings.storagePath.path, "profiles.json"));
    if (await _profilesFile.exists()) {
      final source = await _profilesFile.readAsString();
      final json = json5Decode(source) as Map<String, dynamic>;
      _profiles = ProfileData.fromJson(json);
      if (_profiles.profiles.isEmpty) _profiles.profiles.add(Profile("Default"));
      if (_profiles.active < 0) _profiles.active = 0;
      if (_profiles.active >= _profiles.profiles.length) _profiles.active = _profiles.profiles.length - 1;
    } else {
      _profiles = ProfileData(0, [ Profile("Default") ]);
    }

    //TODO: Check profiles

    _log.fine("Checking for RAR support");
    final extensions = {
      "tar.gz",
      "tgz",
      "tar.bz2",
      "tbz",
      "tar.xz",
      "txz",
      "tar",
      "zip",
    };

    ProcessResult? unrarRes;
    try {
      unrarRes = await Process.run(
        Platform.isWindows ? "unrar.exe" : "unrar",
        const [],
        runInShell: true,
      );
    } on ProcessException catch (ex) {
      _log.finer("error looking for `unrar`", ex);
    }

    ProcessResult? $7zipRes;
    try {
      $7zipRes = await Process.run(
        Platform.isWindows ? "7z.exe" : "7z",
        const [],
        runInShell: true,
      );
    } on ProcessException catch (ex) {
      _log.finer("error looking for `7zip`", ex);
    }

    if (unrarRes?.exitCode == 0) {
      _rarHandler ??= _RarHandler.unrar;
      extensions.add("rar");
    } else {
      _log.fine("`unrar` not supported");
    }

    if ($7zipRes?.exitCode == 0) {
      _rarHandler ??= _RarHandler.$7zip;
      _7zSupported = true;
      extensions.add("rar");
      extensions.add("7z");
    } else {
      _log.fine("`7zip` not supported");
    }

    _extensions = extensions.toList(growable: false);
    _log.fine("Set supported extensions");

    _initialized = true;
    progressCallback?.call(ManagerInitProgress.complete);
    _log.info("Initialization complete.");
  }

  Mod? getModByGuid(UuidValue guid) {
    if (!_initialized) throw NotInitializedError("ModManagerService");

    for (final mod in _mods) {
      if (mod.manifest.getIdentifier() == guid) {
        return mod;
      }
    }
    return null;
  }

  Future<void> save() async {
    if (!_initialized) throw NotInitializedError("ModManagerService");

    final object = _profiles.toJson();
    final content = jsonEncode(object);
    await _profilesFile.writeAsString(content);
  }

  bool makeNewProfile(String name, [bool activate = false]) {
    if (!_initialized) throw NotInitializedError("ModManagerService");

    if (_profiles.profiles.any((p) => p.name == name)) return false;

    final profile = Profile(name);
    _profiles.profiles.add(profile);

    if (activate) {
      final index = _profiles.profiles.indexOf(profile);
      _profiles.active = index;
    }

    return true;
  }

  Future<bool> addMod(File archiveFile) async {
    _log.info("Attempting to add mod from \"${path.basename(archiveFile.path)}\".");

    final tmpDir = Directory(path.join(_settings.tempPath.path, path.basenameWithoutExtension(archiveFile.path)));
    _log.fine("Creating clean temporary directory \"${tmpDir.path}\"");
    if (await tmpDir.exists()) await tmpDir.delete(recursive: true);
    await tmpDir.create(recursive: true);

    _log.fine("Extracting archive");
    await _extractFileToDir(archiveFile, tmpDir);

    _log.fine("Reading manifest");
    var manifest = await ModManifest.fromDirectory(tmpDir);

    if (_mods.any((mod) => mod.manifest.getIdentifier() == manifest.getIdentifier())) {
      _log.severe("Mod with guid already exists!");
      await tmpDir.delete(recursive: true);
      return false;
    }
    
    final match = _archiveFileRegex.firstMatch(path.basename(archiveFile.path));
    
    if (manifest case ModManifestLegacy(generated: final gen) when gen && match != null) {
      final name = match.namedGroup("name");
      manifest = manifest.copyWith(newName: name);
    }

    if (manifest.getNexusData() == null && match != null) {
      final data = NexusData(
        generated: true,
        id: int.parse(match.namedGroup("mod_id")!),
        version: match.namedGroup("version"),
        fileId: int.parse(match.namedGroup("file_id")!),
      );
      manifest = switch (manifest) {
        ModManifestLegacy manifest => manifest.copyWith(newNexusData: data),
        ModManifestV1 manifest => manifest.copyWith(newNexusData: data),
      };
    }

    if (!await tmpDir.containsFile("manifest.json")) {
      final json = manifest.toJson();
      final content = jsonEncode(json);
      final file = File(path.join(tmpDir.path, "manifest.json"));
      await file.writeAsString(content);
    }

    final modDir = Directory(path.join(_modsDir.path, manifest.getName()));
    if (await modDir.exists()) {
      _log.warning("Mod directory already exists in storage. Deleting...");
      await modDir.delete(recursive: true);
    }

    _log.fine("Moving mod to storage");
    await tmpDir.copy(modDir);

    _log.fine("Adding mod");
    final mod = Mod(modDir, manifest);
    _mods.add(mod);

    await tmpDir.delete(recursive: true);
    _log.info("Mod added successfully.");
    return true;
  }

  Future<void> removeMod(Mod mod) async {
    //TODO: Remove mod from profiles that include it
    await mod.directory.delete(recursive: true);
    _mods.remove(mod);
  }

  Future<void> purge() async {
    _log.info("Purging...");

    _log.fine("Checking game directory.");
    if (_settings.gamePath == null) throw Exception("Game path is null!");
    final dataDir = await _settings.gamePath!.tryGetDirectory("data");
    if (dataDir == null) throw Exception("Data directory not found!");

    final list = <Future<FileSystemEntity>>[];
    await for (final entry in dataDir.list()) {
      if (entry is! File) continue;
      if (!_patchRegex.hasMatch(path.basename(entry.path))) continue;
      list.add(entry.delete());
    }

    await Future.wait(list);

    _profiles.deployed = null;
    _log.info("Purge complete.");
  }

  Future<void> deploy() async {
    _log.info("Deploying profile \"${activeProfile.name}\"...");

    _log.fine("Checking game directory");
    if (_settings.gamePath == null) throw Exception("Game path is null!");
    final dataDir = await _settings.gamePath!.tryGetDirectory("data");
    if (dataDir == null) throw Exception("Data directory not found!");

    await purge();

    _log.fine("Collecting mods");
    final mods = activeProfile.mods
      .map((data) => (getModByGuid(data.guid), data))
      .where((tuple) => tuple.$1 != null && tuple.$2.enabled)
      .cast<(Mod, ModData)>()
      .toList(growable: false);

    final groups = HashMap<String, List<_PatchFileTriplet>>();
    
    Future<void> addFilesFromDir(Directory dir) async {
      final files = <File>[];
      await for (final entry in dir.list()) {
        if (entry is! File) continue;
        final name = path.basename(entry.path);
        if (!_patchFileRegex.hasMatch(name)) continue;
        _log.fine("Adding file \"${entry.path}\"");
        files.add(entry);
      }

      final names = <String>{};
      for (final file in files) {
        final name = path.basename(file.path);
        names.add(name.substring(0, 16));
      }

      for (final name in names) {
        final indexes = <int>{};
        for (final file in files) {
          var match = _patchIndexRegex.firstMatch(path.basename(file.path));
          if (match == null) continue;
          indexes.add(int.parse(match[1]!));
        }

        for (final index in indexes) {
          final patchFile = await dir.tryGetFile("$name.patch_$index");
          final gpuFile = await dir.tryGetFile("$name.patch_$index.gpu_resources");
          final streamFile = await dir.tryGetFile("$name.patch_$index.stream");
          
          if (!groups.containsKey(name)) groups[name] = [];
          groups[name]!.add((
            patch: patchFile,
            gpuResources: gpuFile,
            stream: streamFile,
          ));
        }
      }
    }
    
    _log.fine("Grouping files");
    for (final (mod, data) in mods) {
      _log.fine("Working on \"${mod.manifest.getName()}\"");

      switch (mod.manifest) {
        case ModManifestLegacy manifest:
          {
            _log.fine("Legacy manifest found");
            
            if (manifest.options != null) {
              if (data.selected.length != 1) {
                _log.severe("Options have the wrong count!");
                continue;
              }

              final dir = Directory(path.join(mod.directory.path, manifest.options![data.selected[0]]));
              await addFilesFromDir(dir);
            } else {
              await addFilesFromDir(mod.directory);
            }
          }
          break;

        case ModManifestV1 manifest:
          {
            _log.fine("V1 manifest found");
            
            if (manifest.options != null) {
              if (data.toggled.length != manifest.options!.length) {
                _log.severe("Toggled option counts are not equal!");
                continue;
              }

              if (data.selected.length != manifest.options!.length) {
                _log.severe("Selected option counts are not equal!");
                continue;
              }

              _log.fine("Making include list");
              for (int i = 0; i < data.toggled.length; i++) {
                if (!data.toggled[i]) continue;

                final opt = manifest.options![i];

                if (opt.include case List<String> incs) {
                  for (final inc in incs) {
                    final dir = Directory(path.join(mod.directory.path, inc));
                    _log.fine("Adding \"${dir.path}\"");
                    await addFilesFromDir(dir);
                  }
                }

                if (opt.subOptions case List<ModSubOption> subs) {
                  final sub = subs[data.selected[i]];
                  for (final inc in sub.include) {
                    final dir = Directory(path.join(mod.directory.path, inc));
                    _log.fine("Adding \"${dir.path}\"");
                    await addFilesFromDir(dir);
                  }
                }
              }
            } else {
              await addFilesFromDir(mod.directory);
            }
          }
          break;
      }
    }

    _log.fine("Copying files");
    for (final MapEntry(key: name, value: list) in groups.entries) {
      int offset = 0;
      if (_settings.skipList.contains(name)) offset = 1;

      for (int i = 0; i < list.length; i++) {
        final triplet = list[i];
        final index = i + offset;

        final newPatchPath = path.join(dataDir.path, "$name.patch_$index");
        if (triplet.patch != null) {
          await triplet.patch!.copy(newPatchPath);
        } else {
          await File(newPatchPath).create();
        }

        final newGpuResourcesPath = path.join(dataDir.path, "$name.patch_$index.gpu_resources");
        if (triplet.gpuResources != null) {
          await triplet.gpuResources!.copy(newGpuResourcesPath);
        } else {
          await File(newGpuResourcesPath).create();
        }

        final newStreamPath = path.join(dataDir.path, "$name.patch_$index.stream");
        if (triplet.stream != null) {
          await triplet.stream!.copy(newStreamPath);
        } else {
          await File(newStreamPath).create();
        }
      }
    }

    _profiles.deployed = _profiles.active;
    await save();
    
    _log.info("Deployment successful.");
  }

  Profile? addProfile(String name) {
    if (_profiles.profiles.any((p) => p.name == name)) return null;
    final profile = Profile(name);
    _profiles.profiles.add(profile);
    return profile;
  }

  void removeProfile(Profile profile) {
    if (!_profiles.profiles.remove(profile)) return;
    if (_profiles.profiles.isEmpty) {
      _profiles.profiles.add(Profile("Default"));
      return;
    }
    _profiles.active--;
    if (_profiles.active < 0) _profiles.active = 0;
    if (_profiles.active >= _profiles.profiles.length) _profiles.active = _profiles.profiles.length - 1;
  }

    Future<void> _ensureUserRWPermissions(Directory dir) async {
    if (Platform.isWindows) return; // Windows does not support chmod, but this shouldn't be needed on Windows anyway.
    await for (final entity in dir.list(recursive: true, followLinks: false)) {
      try {
        if (entity is File) {
          // 0o600 = rw-------
          await Process.run('chmod', ['600', entity.path]);
        } else if (entity is Directory) {
          // 0o700 = rwx------
          await Process.run('chmod', ['700', entity.path]);
        }
      } catch (e) {
        _log.warning("Failed to set permissions for ${entity.path}: $e");
      }
    }
  }

  Future<void> _extractFileToDir(File archiveFile, Directory targetDir) async {
    if (path.extension(archiveFile.path) == ".rar") {
      if (_rarHandler == null) throw Exception("`rar` format not supported!");

      late final ProcessResult result;
      switch (_rarHandler!) {
        case _RarHandler.$7zip:
          result = await Process.run(
            Platform.isWindows ? "7z.exe" : "7z",
            [
              "x",
              "-o${targetDir.path}",
              archiveFile.path,
            ],
            runInShell: true,
            stdoutEncoding: Utf8Codec(),
          );
          break;
        case _RarHandler.unrar:
          result = await Process.run(
            Platform.isWindows ? "unrar.exe" : "unrar",
            [
              "x",
              archiveFile.path,
              targetDir.path,
            ],
            runInShell: true,
            stdoutEncoding: Utf8Codec(),
          );
          break;
      }

      if (result.exitCode != 0) {
        final out = result.stdout as String;
        final err = result.stderr as String;
        final message = "Special extraction (rar) failed!\nCode: ${result.exitCode}\nStdOut:\n$out\nStdErr:\n$err";
        _log.severe(message);
        throw Exception(message);
      }
    } else if (path.extension(archiveFile.path) == ".7z") {
      if (!_7zSupported) throw Exception("`7z` format not supported!");

      final result = await Process.run(
        Platform.isWindows ? "7z.exe" : "7z",
        [
          "x",
          "-o${targetDir.path}",
          archiveFile.path,
        ],
        runInShell: true,
        stdoutEncoding: Utf8Codec(),
      );

      if (result.exitCode != 0) {
        final out = result.stdout as String;
        final err = result.stderr as String;
        final message = "Special extraction (7z) failed!\nCode: ${result.exitCode}\nStdOut:\n$out\nStdErr:\n$err";
        _log.severe(message);
        throw Exception(message);
      }
    } else {
      await extractFileToDisk(archiveFile.path, targetDir.path);
    }

    await _ensureUserRWPermissions(targetDir);
  }
}