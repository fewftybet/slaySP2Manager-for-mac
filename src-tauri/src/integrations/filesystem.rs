use std::fs;
use std::path::{Path, PathBuf};

pub fn game_executable_path(path: &Path) -> Option<PathBuf> {
    // macOS: Check for .app bundle structure
    #[cfg(target_os = "macos")]
    {
        // Case 1: Path is already inside .app/Contents/MacOS
        // e.g., /Users/xxx/Library/Application Support/Steam/steamapps/common/Slay the Spire 2/SlayTheSpire2.app/Contents/MacOS
        let path_str = path.to_string_lossy();
        if path_str.contains(".app/Contents/MacOS") {
            // Try direct executable in current directory
            let direct = path.join("SlayTheSpire2");
            if direct.is_file() {
                return Some(direct);
            }
            let direct_spaced = path.join("Slay the Spire 2");
            if direct_spaced.is_file() {
                return Some(direct_spaced);
            }
        }
        
        // Case 2: Path is the game root containing .app bundle
        // e.g., /Users/xxx/Library/Application Support/Steam/steamapps/common/Slay the Spire 2
        // Try standard .app bundle path
        let app_bundle = path.join("SlayTheSpire2.app");
        let app_executable = app_bundle.join("Contents/MacOS/SlayTheSpire2");
        if app_executable.is_file() {
            return Some(app_executable);
        }
        
        // Try with space in name
        let app_bundle_spaced = path.join("Slay the Spire 2.app");
        let app_executable_spaced = app_bundle_spaced.join("Contents/MacOS/Slay the Spire 2");
        if app_executable_spaced.is_file() {
            return Some(app_executable_spaced);
        }
        
        // Case 3: Check if we're at Contents level
        let contents_check = path.join("MacOS/SlayTheSpire2");
        if contents_check.is_file() {
            return Some(contents_check);
        }
        let contents_check_spaced = path.join("MacOS/Slay the Spire 2");
        if contents_check_spaced.is_file() {
            return Some(contents_check_spaced);
        }
    }

    // Windows/Linux: Check for direct executable
    #[cfg(not(target_os = "macos"))]
    {
        return [
            "SlayTheSpire2",
            "Slay the Spire 2",
            "SlayTheSpire2.exe",
            "Slay the Spire 2.exe",
        ]
        .into_iter()
        .map(|name| path.join(name))
        .find(|candidate| candidate.is_file());
    }
    
    None
}

/// Returns the game root directory from an executable path.
/// On macOS, if the executable is inside an .app bundle, returns the .app bundle's parent directory.
pub fn game_root_from_exe_path(exe_path: &Path) -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        // If path contains ".app/Contents/MacOS", extract the .app bundle's parent
        let path_str = exe_path.to_string_lossy();
        if let Some(app_pos) = path_str.find(".app/Contents/MacOS") {
            // Find the start of the .app name
            if let Some(separator_pos) = path_str[..app_pos].rfind('/') {
                return PathBuf::from(&path_str[..separator_pos + 1]);
            }
        }
    }
    
    // For non-macOS or non-.app paths, return parent directory
    exe_path.parent().unwrap_or(exe_path).to_path_buf()
}

pub fn contains_game_executable(path: &Path) -> bool {
    game_executable_path(path).is_some()
}

pub fn list_directories(path: &Path) -> Vec<std::path::PathBuf> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect(),
        Err(_) => Vec::new(),
    }
}
