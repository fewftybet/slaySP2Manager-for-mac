use std::path::{Path, PathBuf};

pub fn slay_spire_2_save_root() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        return appdata_root().map(|root| slay_spire_2_save_root_from_appdata(&root));
    }

    #[cfg(not(target_os = "windows"))]
    {
        data_root().map(|root| slay_spire_2_save_root_from_data_dir(&root))
    }
}

pub fn manager_data_root() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        return appdata_root().map(|root| manager_data_root_from_base(&root));
    }

    #[cfg(not(target_os = "windows"))]
    {
        data_root().map(|root| manager_data_root_from_base(&root))
    }
}

pub fn manager_save_backups_root() -> Result<PathBuf, String> {
    Ok(manager_data_root()?.join("backups").join("saves"))
}

pub fn manager_cloud_backups_root() -> Result<PathBuf, String> {
    Ok(manager_data_root()?.join("backups").join("cloud_cache"))
}

#[cfg(target_os = "windows")]
fn appdata_root() -> Result<PathBuf, String> {
    std::env::var("APPDATA")
        .map(PathBuf::from)
        .map_err(|_| "APPDATA not available".to_string())
}

fn data_root() -> Result<PathBuf, String> {
    dirs_next::data_dir().ok_or_else(|| "data directory not available".to_string())
}

#[cfg(any(target_os = "windows", test))]
fn slay_spire_2_save_root_from_appdata(appdata: &Path) -> PathBuf {
    appdata.join("SlayTheSpire2").join("steam")
}

fn slay_spire_2_save_root_from_data_dir(data_dir: &Path) -> PathBuf {
    data_dir.join("SlayTheSpire2").join("steam")
}

fn manager_data_root_from_base(base: &Path) -> PathBuf {
    base.join("SlaySP2Manager")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_linux_save_root_from_data_dir() {
        let root = slay_spire_2_save_root_from_data_dir(Path::new("/home/test/.local/share"));
        assert_eq!(root, PathBuf::from("/home/test/.local/share/SlayTheSpire2/steam"));
    }

    #[test]
    fn builds_windows_save_root_from_appdata() {
        let root = slay_spire_2_save_root_from_appdata(Path::new(r"C:\Users\test\AppData\Roaming"));
        assert_eq!(
            root,
            PathBuf::from(r"C:\Users\test\AppData\Roaming").join("SlayTheSpire2").join("steam")
        );
    }

    #[test]
    fn builds_manager_data_root_from_base() {
        let root = manager_data_root_from_base(Path::new("/home/test/.local/share"));
        assert_eq!(root, PathBuf::from("/home/test/.local/share/SlaySP2Manager"));
    }
}
