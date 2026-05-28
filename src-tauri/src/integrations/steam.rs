use std::fs;
use std::path::PathBuf;

use crate::domain::game::GameDetectSource;
use crate::integrations::filesystem::contains_game_executable;

const GAME_FOLDER_NAME: &str = "Slay the Spire 2";
const STEAM_APP_ID: &str = "2868840";

pub fn find_game_install() -> Option<(PathBuf, GameDetectSource)> {
    if let Some(steam_root) = read_steam_root() {
        let default_game = steam_root
            .join("steamapps")
            .join("common")
            .join(GAME_FOLDER_NAME);
        if contains_game_executable(&default_game) {
            return Some((default_game, GameDetectSource::SteamDefault));
        }

        let vdf_path = steam_root.join("steamapps").join("libraryfolders.vdf");
        if let Some(libraries) = parse_library_folders(&vdf_path) {
            for lib_path in libraries {
                let game_path = lib_path
                    .join("steamapps")
                    .join("common")
                    .join(GAME_FOLDER_NAME);
                if contains_game_executable(&game_path) {
                    return Some((game_path, GameDetectSource::SteamLibrary));
                }
            }
        }
    }

    for (candidate, source) in common_candidates() {
        if contains_game_executable(&candidate) {
            return Some((candidate, source));
        }
    }

    None
}

fn read_steam_root() -> Option<PathBuf> {
    read_steam_roots().into_iter().next()
}

fn read_steam_roots() -> Vec<PathBuf> {
    steam_root_candidates()
        .into_iter()
        .filter(|path| path.exists())
        .collect()
}

fn steam_root_candidates() -> Vec<PathBuf> {
    let Some(home) = dirs_next::home_dir() else {
        return Vec::new();
    };

    vec![
        home.join(".steam/steam"),
        home.join(".local/share/Steam"),
        home.join(".var/app/com.valvesoftware.Steam/.steam/steam"),
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
    ]
}

fn parse_library_folders(vdf_path: &PathBuf) -> Option<Vec<PathBuf>> {
    let content = fs::read_to_string(vdf_path).ok()?;
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut prioritized: Vec<PathBuf> = Vec::new();

    let mut current_path: Option<String> = None;
    let mut has_our_app = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("\"path\"") {
            if let Some(value) = extract_vdf_value(trimmed) {
                current_path = Some(value);
                has_our_app = false;
            }
        }

        if trimmed.contains(&format!("\"{}\"", STEAM_APP_ID)) {
            has_our_app = true;
        }

        if trimmed == "}" {
            if let Some(ref path) = current_path {
                let pb = PathBuf::from(path);
                if pb.exists() {
                    if has_our_app {
                        prioritized.push(pb);
                    } else {
                        paths.push(pb);
                    }
                }
                current_path = None;
                has_our_app = false;
            }
        }
    }

    prioritized.extend(paths);
    if prioritized.is_empty() {
        None
    } else {
        Some(prioritized)
    }
}

fn extract_vdf_value(line: &str) -> Option<String> {
    let all_quotes: Vec<&str> = line.split('"').collect();
    if all_quotes.len() >= 4 {
        let value = all_quotes[3].replace("\\\\", "\\");
        if !value.is_empty() {
            return Some(value);
        }
    }
    None
}

fn common_candidates() -> Vec<(PathBuf, GameDetectSource)> {
    let mut result = Vec::new();
    let Some(home) = dirs_next::home_dir() else {
        return result;
    };

    for base in [
        home.join(".steam/steam/steamapps/common"),
        home.join(".local/share/Steam/steamapps/common"),
        home.join(".var/app/com.valvesoftware.Steam/.steam/steam/steamapps/common"),
        home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam/steamapps/common"),
        home.join("Games"),
    ] {
        result.push((base.join(GAME_FOLDER_NAME), GameDetectSource::SteamDefault));
    }

    result
}

pub fn get_current_steam_account_id() -> Option<u32> {
    find_active_cloud_app_dir()
        .map(|(account_id, _)| account_id)
        .or_else(|| {
            read_steam_roots().into_iter().find_map(|steam_root| {
                let vdf_path = steam_root.join("config").join("loginusers.vdf");
                parse_loginusers_for_account_id(&vdf_path)
            })
        })
}

fn parse_loginusers_for_account_id(vdf_path: &PathBuf) -> Option<u32> {
    let content = fs::read_to_string(vdf_path).ok()?;
    parse_loginusers_content_for_account_id(&content)
}

fn parse_loginusers_content_for_account_id(content: &str) -> Option<u32> {
    parse_loginusers_content_for_account_ids(content).into_iter().next()
}

fn parse_loginusers_content_for_account_ids(content: &str) -> Vec<u32> {
    let mut ids = Vec::new();
    let mut current_steamid64: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() == 19 {
            let id = trimmed.trim_matches('"');
            if id.starts_with("7656") && id.len() == 17 {
                current_steamid64 = Some(id.to_string());
            }
        }

        if trimmed.contains("\"MostRecent\"") && trimmed.contains("\"1\"") {
            if let Some(account_id) = current_steamid64
                .as_deref()
                .and_then(steam_id64_to_account_id)
            {
                ids.insert(0, account_id);
            }
        } else if trimmed == "}" {
            if let Some(account_id) = current_steamid64
                .as_deref()
                .and_then(steam_id64_to_account_id)
            {
                if !ids.contains(&account_id) {
                    ids.push(account_id);
                }
            }
            current_steamid64 = None;
        }
    }

    ids
}

fn steam_id64_to_account_id(id: &str) -> Option<u32> {
    let id_u64 = id.parse::<u64>().ok()?;
    Some((id_u64 - 76561197960265728) as u32)
}

fn find_active_cloud_app_dir() -> Option<(u32, PathBuf)> {
    for steam_root in read_steam_roots() {
        let vdf_path = steam_root.join("config").join("loginusers.vdf");
        let Ok(content) = fs::read_to_string(vdf_path) else {
            continue;
        };

        for account_id in parse_loginusers_content_for_account_ids(&content) {
            let app_dir = steam_root
                .join("userdata")
                .join(account_id.to_string())
                .join(STEAM_APP_ID);
            if app_dir.exists() {
                return Some((account_id, app_dir));
            }
        }
    }

    None
}

pub fn find_cloud_app_dir() -> Option<PathBuf> {
    find_active_cloud_app_dir().map(|(_, app_dir)| app_dir)
}

pub fn find_cloud_save_dir() -> Option<PathBuf> {
    let remote_dir = find_cloud_app_dir()?.join("remote");
    if remote_dir.exists() {
        Some(remote_dir)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOGINUSERS_MOST_RECENT: &str = r#"
"users"
{
    "76561198000000001"
    {
        "AccountName" "user1"
        "MostRecent" "0"
    }
    "76561199516809413"
    {
        "AccountName" "user2"
        "MostRecent" "1"
    }
}
"#;

    const LOGINUSERS_NO_MOST_RECENT: &str = r#"
"users"
{
    "76561199516809413"
    {
        "AccountName" "user2"
        "MostRecent" "0"
    }
}
"#;

    #[test]
    fn prefers_most_recent_user() {
        let ids = parse_loginusers_content_for_account_ids(LOGINUSERS_MOST_RECENT);
        // MostRecent user (76561199516809413 - 76561197960265728 = 1556543685) should be first
        assert_eq!(ids[0], 1556543685u32);
    }

    #[test]
    fn falls_back_to_only_user_when_no_most_recent() {
        let id = parse_loginusers_content_for_account_id(LOGINUSERS_NO_MOST_RECENT);
        assert_eq!(id, Some(1556543685u32));
    }

    #[test]
    fn steam_id64_conversion_is_correct() {
        assert_eq!(steam_id64_to_account_id("76561199516809413"), Some(1556543685u32));
        assert_eq!(steam_id64_to_account_id("76561197960265728"), Some(0u32));
    }
}
