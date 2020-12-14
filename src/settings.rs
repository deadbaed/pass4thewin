use crate::constants::{ID_APPLICATION, ID_ORGANIZATION, ID_QUALIFIER, SETTINGS_FILENAME};
use anyhow::Context;
use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip)]
    path: Option<PathBuf>,

    password_store: PathBuf,
    pgp_key: PathBuf,
}

impl Settings {
    fn from_path(path: &Path) -> anyhow::Result<Self> {
        let mut file = File::open(path)
            .with_context(|| format!("Failed to open settings file '{}'", path.display()))?;

        let mut settings_raw = String::new();
        file.read_to_string(&mut settings_raw)
            .context("Failed to load settings file")?;

        // Import settings
        let mut settings: Settings = toml::from_str(&settings_raw).context(format!(
            "Failed to load settings from file '{}'.\nPlease check the syntax of the settings file try again.",
            path.display()
        ))?;

        // Is used to write to the same file that was loaded
        settings.path = Some(PathBuf::from(path));

        Ok(settings)
    }

    /// look for settings file in folder where binary is ran from
    fn from_binary_path() -> anyhow::Result<Self> {
        let path = construct_path_from_binary_path()?;
        Self::from_path(&path)
    }

    /// look for settings file in `%APPDATA%\ID_ORGANIZATION\ID_APPLICATION\config`
    fn from_roaming_app_data() -> anyhow::Result<Self> {
        let path = construct_path_from_app_data()?;
        Self::from_path(&path)
    }

    /// Convert `Settings` struct and write it out to a TOML file
    fn write_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string(self)?;

        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// Write settings to file
    pub fn write(&mut self) -> anyhow::Result<()> {
        match &self.path {
            // If we already know where the settings file is, write it directly
            Some(path) => self.write_to_file(&path),

            // If there is no existing settings file, use Windows AppData folder to store settings
            None => match construct_path_from_app_data() {
                Ok(path) => {
                    println!("Saving settings to '{}'", path.display());

                    // Create folder if it doesn't exist
                    let parent = path.parent().context("Failed to find parent folder")?;
                    std::fs::create_dir_all(parent)?;

                    // Write to file
                    self.write_to_file(&path)
                }
                Err(e) => Err(e),
            },
        }
    }

    /// Try to load user settings in this order:
    ///
    /// - where the executable is ran from
    /// - AppData Roaming of current Windows user
    ///
    /// On failure, create an empty `Settings` struct
    pub fn try_load() -> Self {
        if let Ok(settings) = Self::from_binary_path() {
            return settings;
        };
        if let Ok(settings) = Self::from_roaming_app_data() {
            return settings;
        };
        Self::default()
    }

    pub fn set_password_store_path(&mut self, path: &Path) {
        self.password_store = PathBuf::from(path);
    }

    pub fn set_pgp_key_path(&mut self, path: &Path) {
        self.pgp_key = PathBuf::from(path);
    }
}

/// generate settings path based off current binary's path
///
/// example: BINARY_PATH/pass4thewin.toml
fn construct_path_from_binary_path() -> anyhow::Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.set_file_name(SETTINGS_FILENAME);

    Ok(path)
}

/// generate settings path based off windows's appdata
///
/// should be located at `%APPDATA%\ID_ORGANIZATION\ID_APPLICATION\config\pass4thewin.toml`
fn construct_path_from_app_data() -> anyhow::Result<PathBuf> {
    let proj_dir = ProjectDirs::from(ID_QUALIFIER, ID_ORGANIZATION, ID_APPLICATION)
        .context("Failed to lookup settings folder in the Windows Known Folder API.")?;

    let mut path = PathBuf::from(proj_dir.config_dir());
    path.push(SETTINGS_FILENAME);

    Ok(path)
}
