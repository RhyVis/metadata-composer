use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use ts_rs::TS;

use crate::core::{Language, check_init_flag, create_init_flag, get_handle_ref};

#[cfg(debug_assertions)]
const CONFIG_FILE_NAME: &str = "Config.dev.toml";
#[cfg(not(debug_assertions))]
const CONFIG_FILE_NAME: &str = "Config.toml";

const DIR_NAME_ARCHIVE: &str = "archive";
const DIR_NAME_IMAGE: &str = "image";

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct AppConfig {
    #[serde(default)]
    lang: Language,
    #[serde(default)]
    path_data: PathBuf,
    #[serde(default)]
    path_deploy: Option<PathBuf>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            lang: Language::default(),
            path_data: PathBuf::from("."),
            path_deploy: None,
        }
    }
}

impl AppConfig {
    pub fn path_data(&self) -> &Path {
        &self.path_data.as_ref()
    }

    pub fn path_deploy(&self) -> Option<&Path> {
        self.path_deploy.as_deref()
    }

    pub fn dir_archive(&self) -> PathBuf {
        self.path_data.join(DIR_NAME_ARCHIVE)
    }

    pub fn dir_image(&self) -> PathBuf {
        self.path_data.join(DIR_NAME_IMAGE)
    }

    fn check(&self) -> Result<()> {
        let path_data = &self.path_data;
        if !path_data.exists() {
            fs::create_dir_all(path_data).map_err(|e| {
                anyhow!(
                    "Root data directory '{}' may not write: {}",
                    path_data.display(),
                    e
                )
            })?;
        }
        if let Some(path_deploy) = &self.path_deploy {
            if !path_deploy.exists() {
                fs::create_dir_all(path_deploy)?;
            }
        }

        let dir_archive = self.dir_archive();
        if !dir_archive.exists() {
            fs::create_dir_all(&dir_archive)?;
        }
        let dir_image = self.dir_image();
        if !dir_image.exists() {
            fs::create_dir_all(&dir_image)?;
        }

        Ok(())
    }

    pub(super) fn update_field(&mut self, name: &str, value: Value) -> Result<()> {
        match name {
            "lang" => {
                let lang = value.into();
                self.write(|c| {
                    c.lang = lang;
                    Ok(())
                })?;
            },
            "path_root" => {
                if let Some(value) = value.as_str() {
                    let path = Path::new(value);
                    if !path.exists() || !path.is_dir() {
                        warn!("Path '{}' does not exist or is not a directory.", value);
                        return Err(anyhow!("Invalid path: {}", value));
                    }
                    self.write(|c| {
                        c.path_data = path.to_path_buf();
                        Ok(())
                    })?;
                    info!("Updated path_data to '{}'", self.path_data.display());
                } else {
                    warn!(
                        "Invalid value for path_data: expected a string, got {:?}",
                        value
                    );
                    return Err(anyhow!("Invalid value for path_data"));
                }
            },
            "path_deploy" => {
                if let Some(_) = value.as_null() {
                    self.write(|c| {
                        c.path_deploy = None;
                        Ok(())
                    })?;
                    info!("Removed path_deploy");
                } else if let Some(value) = value.as_str() {
                    let path = Path::new(value);
                    if !path.exists() || !path.is_dir() {
                        warn!("Path '{}' does not exist or is not a directory.", value);
                        return Err(anyhow!("Invalid path: {}", value));
                    }
                    self.write(|c| {
                        c.path_deploy = Some(path.to_path_buf());
                        Ok(())
                    })?;
                    info!(
                        "Updated path_deploy to '{}'",
                        self.path_deploy.as_ref().unwrap().display()
                    );
                } else {
                    warn!(
                        "Invalid value for path_deploy: expected a string or null, got {:?}",
                        value
                    );
                    return Err(anyhow!("Invalid value for path_deploy"));
                }
            },
            _ => {
                warn!("Unknown field name {} with value {}", name, value);
                return Err(anyhow!("Unknown field name {}", name));
            },
        }
        Ok(())
    }

    pub(super) fn load(app: &AppHandle) -> Result<Self> {
        let config_path = app.path().app_config_dir()?.join(CONFIG_FILE_NAME);
        info!("Loading configuration from '{}'", config_path.display());

        let config = match fs::read_to_string(&config_path) {
            Ok(content) => toml::from_str(&content).map_err(|e| {
                anyhow!(
                    "Failed to parse config file '{}': {}",
                    config_path.display(),
                    e
                )
            })?,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    info!(
                        "Config file '{}' not found, using default configuration.",
                        config_path.display()
                    );

                    let mut default_config = Self::default();
                    if !check_init_flag(app)? {
                        info!("Init flag not set, asking for path_root selection.");
                        app.dialog()
                            .message(
                                "The application is not initialized.\n\
                            Please select the root path for the application data.",
                            )
                            .title("Initialization Required")
                            .blocking_show();
                        let path_chosen = app
                            .dialog()
                            .file()
                            .set_title("Choose root path for data")
                            .blocking_pick_folder()
                            .and_then(|path| path.into_path().ok());

                        if let Some(path_root_alt) = path_chosen {
                            info!("User selected root path: {}", path_root_alt.display());
                            default_config.path_data = path_root_alt;
                        } else {
                            return Err(anyhow!(
                                "No root path selected, cannot initialize application."
                            ));
                        }

                        create_init_flag(app)?;
                    }

                    default_config.save(app)?;

                    default_config
                },
                _ => return Err(error.into()),
            },
        };
        config
            .check()
            .map_err(|e| anyhow!("Configuration check failed: {}", e))?;

        Ok(config)
    }

    fn save(&self, app: &AppHandle) -> Result<()> {
        let mut config_path = app.path().app_config_dir()?;
        if !config_path.exists() {
            fs::create_dir_all(&config_path)?;
        }
        config_path.push(CONFIG_FILE_NAME);
        fs::write(&config_path, toml::to_string_pretty(self)?)?;
        info!("Configuration saved to '{}'", config_path.display());
        Ok(())
    }

    fn write(&mut self, f: impl FnOnce(&mut Self) -> Result<()>) -> Result<()> {
        f(self)?;
        self.save(get_handle_ref())?;
        Ok(())
    }
}

impl From<Value> for Language {
    fn from(value: Value) -> Self {
        serde_json::from_value::<Language>(value.clone()).unwrap_or_else(|e| {
            warn!("Provided language '{:#}' not parsable: '{}", value, e);
            Language::default()
        })
    }
}
