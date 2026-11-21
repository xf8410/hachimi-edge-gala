use std::{
    collections::HashSet,
    ffi::CString,
    fs,
    path::{Path, PathBuf},
};

use crate::core::{plugin_api::Plugin, Hachimi};

pub fn load_libraries() -> Vec<Plugin> {
    let mut plugins = Vec::new();
    let mut loaded = HashSet::new();
    let config = Hachimi::instance().config.load();
    let names = &config.android.load_libraries;

    if names.is_empty() {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Some(lib_dir) = find_native_lib_dir() {
            candidates = collect_candidate_libs(&lib_dir);
        } else {
            warn!("Failed to locate native lib dir for plugin autoscan");
        }
        if candidates.is_empty() {
            // With extractNativeLibs=false the plugin libs live inside the APK and
            // the native lib dir scan above cannot see them. Fall back to pulling
            // libhachimi_*.so entries out of the APK into a writable dir.
            match find_base_apk_path() {
                Some(apk_path) => {
                    let dest_dir = super::utils::get_game_dir().join("plugins_cache");
                    if let Err(e) = fs::create_dir_all(&dest_dir) {
                        warn!("Failed to create plugin cache dir {}: {}", dest_dir.display(), e);
                    } else {
                        candidates = extract_plugins_from_apk(&apk_path, &dest_dir);
                    }
                }
                None => warn!("Failed to locate base APK for plugin extraction"),
            }
        }
        for entry in candidates {
            let display = entry.display().to_string();
            if loaded.contains(&display) {
                continue;
            }
            if let Some(plugin) = try_load_library(&display) {
                loaded.insert(display);
                plugins.push(plugin);
            }
        }
    } else {
        for name in names.iter() {
            if loaded.contains(name) {
                continue;
            }
            if let Some(plugin) = try_load_library(name) {
                loaded.insert(name.clone());
                plugins.push(plugin);
            }
        }
    }

    plugins
}

fn try_load_library(name_or_path: &str) -> Option<Plugin> {
    let Ok(name_cstr) = CString::new(name_or_path) else {
        warn!("Invalid library name: {}", name_or_path);
        return None;
    };

    let handle = unsafe { libc::dlopen(name_cstr.as_ptr(), libc::RTLD_NOW) };
    if handle.is_null() {
        let err = unsafe { libc::dlerror() };
        if err.is_null() {
            warn!("Failed to load library: {}", name_or_path);
        } else {
            let err = unsafe { std::ffi::CStr::from_ptr(err) };
            warn!(
                "Failed to load library: {} ({})",
                name_or_path,
                err.to_string_lossy()
            );
        }
        return None;
    }

    let init_enum = {
        let v3_addr = unsafe { libc::dlsym(handle, c"hachimi_init_v3".as_ptr()) };
        if !v3_addr.is_null() {
            Some(crate::core::plugin_api::PluginInit::V3(unsafe { std::mem::transmute(v3_addr) }))
        } else {
            let v2_addr = unsafe { libc::dlsym(handle, c"hachimi_init".as_ptr()) };
            if !v2_addr.is_null() {
                Some(crate::core::plugin_api::PluginInit::V2(unsafe { std::mem::transmute(v2_addr) }))
            } else {
                None
            }
        }
    };

    match init_enum {
        Some(init_fn) => {
            info!("Loaded library: {}", name_or_path);
            Some(Plugin {
                name: name_or_path.to_string(),
                init_fn,
            })
        }
        None => {
            warn!("Library loaded but missing hachimi_init: {}", name_or_path);
            unsafe {
                libc::dlclose(handle);
            }
            None
        }
    }
}

fn find_native_lib_dir() -> Option<PathBuf> {
    let maps = fs::read_to_string("/proc/self/maps").ok()?;
    for line in maps.lines() {
        let Some(path) = line.split_whitespace().last() else {
            continue;
        };
        if path.ends_with("/libmain.so") {
            return Path::new(path).parent().map(Path::to_path_buf);
        }
    }
    None
}

const AUTOSCAN_PREFIX: &str = "libhachimi_";

fn collect_candidate_libs(lib_dir: &Path) -> Vec<PathBuf> {
    let mut libs = Vec::new();
    let Ok(entries) = fs::read_dir(lib_dir) else {
        return libs;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|v| v.to_str()) else {
            continue;
        };
        if !file_name.starts_with(AUTOSCAN_PREFIX) || !file_name.ends_with(".so") {
            continue;
        }
        libs.push(path);
    }
    libs
}

const APK_PLUGIN_ENTRY_PREFIX: &str = "lib/arm64-v8a/libhachimi_";

/// Locate the real base.apk file path from /proc/self/maps.
/// Inside-APK native libs are mapped as "<apk>!/lib/..."; matching that form
/// first still finds the APK when libs are mapped directly from it.
fn find_base_apk_path() -> Option<PathBuf> {
    let maps = fs::read_to_string("/proc/self/maps").ok()?;
    for line in maps.lines() {
        let Some(path) = line.split_whitespace().last() else {
            continue;
        };
        if let Some(apk) = path.strip_suffix("!/lib/arm64-v8a/libmain.so") {
            return Some(PathBuf::from(apk));
        }
        if path.ends_with("/base.apk") {
            return Some(PathBuf::from(path));
        }
    }
    None
}

/// Extract libhachimi_*.so entries from the APK into dest_dir, return their paths.
fn extract_plugins_from_apk(apk_path: &Path, dest_dir: &Path) -> Vec<PathBuf> {
    let mut extracted = Vec::new();
    let file = match fs::File::open(apk_path) {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to open APK for plugin extraction: {} ({})", apk_path.display(), e);
            return extracted;
        }
    };
    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            warn!("Failed to read APK as zip: {} ({})", apk_path.display(), e);
            return extracted;
        }
    };

    for i in 0..archive.len() {
        let Ok(mut entry) = archive.by_index(i) else { continue };
        let Some(name) = entry.enclosed_name() else { continue };
        let Some(name_str) = name.to_str() else { continue };
        if !name_str.starts_with(APK_PLUGIN_ENTRY_PREFIX) || !name_str.ends_with(".so") {
            continue;
        }
        let Some(file_name) = name_str.rsplit('/').next().map(|v| v.to_string()) else {
            continue;
        };
        let dest = dest_dir.join(&file_name);
        let mut out = match fs::File::create(&dest) {
            Ok(o) => o,
            Err(e) => {
                warn!("Failed to create plugin cache file {}: {}", dest.display(), e);
                continue;
            }
        };
        if let Err(e) = std::io::copy(&mut entry, &mut out) {
            warn!("Failed to extract plugin {}: {}", file_name, e);
            continue;
        }
        info!("Extracted plugin from APK: {}", dest.display());
        extracted.push(dest);
    }
    extracted
}
