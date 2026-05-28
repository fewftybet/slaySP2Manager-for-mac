use std::fs;
use std::path::{Path, PathBuf};

pub fn game_executable_path(path: &Path) -> Option<PathBuf> {
    [
        "SlayTheSpire2",
        "Slay the Spire 2",
        "SlayTheSpire2.exe",
        "Slay the Spire 2.exe",
    ]
    .into_iter()
    .map(|name| path.join(name))
    .find(|candidate| candidate.is_file())
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
