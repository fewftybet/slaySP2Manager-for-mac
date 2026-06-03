use std::path::PathBuf;

use crate::app::state::AppSettings;
use crate::domain::game::{GameDetectSource, GameInstall};
use crate::integrations::filesystem::{contains_game_executable, game_executable_path, game_root_from_exe_path};
use crate::integrations::steam::find_game_install;
use crate::utils::error::AppError;

#[derive(Debug, Clone)]
pub struct GameService {
    settings: AppSettings,
}

impl GameService {
    pub fn new(settings: AppSettings) -> Self {
        Self { settings }
    }

    pub fn detect_install(&self) -> Result<GameInstall, AppError> {
        if let Some(path) = self.settings.game_root_dir.as_ref() {
            let root = PathBuf::from(path);
            if contains_game_executable(&root) {
                return Ok(self.build_install(root, GameDetectSource::Config));
            }
        }

        if let Some((root, source)) = find_game_install() {
            return Ok(self.build_install(root, source));
        }

        Err(AppError::GameNotFound)
    }

    fn build_install(&self, root: PathBuf, detected_by: GameDetectSource) -> GameInstall {
        let disabled_name = self.settings.disabled_mods_dir_name.as_str();
        let exe_path = game_executable_path(&root)
            .unwrap_or_else(|| root.join("SlayTheSpire2"));
        
        // On macOS, if executable is inside .app bundle, extract the actual game root
        let game_root = if cfg!(target_os = "macos") {
            game_root_from_exe_path(&exe_path)
        } else {
            root.clone()
        };
        
        // On macOS, mods directory is inside .app/Contents/MacOS/mods
        let mods_dir = if cfg!(target_os = "macos") {
            exe_path.parent().unwrap_or(&exe_path).join("mods")
        } else {
            game_root.join("mods")
        };
        
        let disabled_mods_dir = if cfg!(target_os = "macos") {
            exe_path.parent().unwrap_or(&exe_path).join(disabled_name)
        } else {
            game_root.join(disabled_name)
        };
        
        GameInstall {
            root_dir: game_root.to_string_lossy().to_string(),
            exe_path: exe_path.to_string_lossy().to_string(),
            mods_dir: mods_dir.to_string_lossy().to_string(),
            disabled_mods_dir: disabled_mods_dir.to_string_lossy().to_string(),
            detected_by,
            is_valid: true,
        }
    }
}
