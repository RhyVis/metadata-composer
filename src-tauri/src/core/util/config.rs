use crate::core::util::APP_ROOT;
use anyhow::{Result, anyhow};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock, RwLockReadGuard};
use ts_rs::TS;

const CONFIG_FILE_NAME: &str = "config.toml";

static CONFIG_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn config_path() -> &'static Path {
    CONFIG_PATH
        .get()
        .expect("Config file not initialized")
        .as_ref()
}

const DIR_NAME_ARCHIVE: &str = "archive";
const DIR_NAME_IMAGE: &str = "image";

static CONFIG: OnceLock<RwLock<InternalConfig>> = OnceLock::new();

pub fn init_config(config_dir: PathBuf, root_path: Option<PathBuf>) -> Result<()> {
    let config_path = config_dir.join(CONFIG_FILE_NAME);
    CONFIG_PATH
        .set(config_path.to_owned())
        .map_err(|_| anyhow!("Config path already initialized"))?;

    info!("Trying to load config from: {}", config_path.display());
    let config: InternalConfig = if !config_path.exists() {
        warn!("Config file does not exist at {}", config_path.display());
        let mut default_config = RawConfig::default();
        if let Some(parent_path) = config_path.parent() {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path)?;
            }
        }
        if let Some(initial_root_path) = root_path {
            info!(
                "Providing initial root path: {}",
                initial_root_path.display()
            );
            default_config.root_data = Some(initial_root_path.display().to_string());
        }

        fs::write(
            &config_path,
            format!(
                "# Config File\n\n{}",
                toml::to_string_pretty(&InternalConfig::from(default_config.clone()))
                    .map_err(|e| anyhow!("Failed to serialize default config: {}", e))?
            ),
        )
        .map_err(|e| {
            anyhow!(
                "Failed to write default config to {}: {}",
                config_path.display(),
                e
            )
        })?;

        info!("Created default config file at: {}", config_path.display());

        default_config
    } else {
        let content = fs::read_to_string(config_path)
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        toml::from_str(&content).map_err(|e| anyhow!("Failed to parse config file: {}", e))?
    }
    .into();

    config
        .check()
        .map_err(|e| anyhow!("Config check failed: {}", e))?;

    CONFIG
        .set(RwLock::new(config))
        .map_err(|_| anyhow!("Config already initialized"))?;

    Ok(())
}

pub fn get_config() -> Result<RwLockReadGuard<'static, InternalConfig>> {
    CONFIG
        .get()
        .expect("Config not initialized")
        .read()
        .map_err(|_| anyhow!("Failed to lock config for reading"))
}

pub fn get_config_copy() -> Result<InternalConfig> {
    get_config().map(|config| config.clone())
}

pub fn update_config(f: impl FnOnce(&mut InternalConfig)) -> Result<()> {
    let mut config = CONFIG
        .get()
        .expect("Config not initialized")
        .write()
        .map_err(|_| anyhow!("Failed to lock config for writing"))?;
    f(&mut config);
    info!("Updating config: {:?}", config);
    config.save()?;
    Ok(())
}

pub fn update_config_field(name: String, value: Option<String>) -> Result<()> {
    match name.as_str() {
        "root_data" if value.is_some() && !value.clone().unwrap().is_empty() => {
            update_config(|config| {
                config.root_data = value.unwrap().into();
            })?;
            Ok(())
        }
        "root_deploy" => update_config(|config| {
            config.root_deploy = if let Some(value) = value {
                if !value.is_empty() {
                    Some(PathBuf::from(value))
                } else {
                    None
                }
            } else {
                None
            };
        }),
        _ => {
            warn!(
                "Unknown field name {name} with value {}",
                value.unwrap_or("None".to_string())
            );
            Err(anyhow!("Unknown field name {}", name))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RawConfig {
    #[serde(default)]
    root_data: Option<String>,
    #[serde(default)]
    root_deploy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct InternalConfig {
    root_data: PathBuf,
    root_deploy: Option<PathBuf>,
}

impl InternalConfig {
    pub fn root_data(&self) -> &PathBuf {
        &self.root_data
    }

    pub fn root_deploy(&self) -> Option<&PathBuf> {
        self.root_deploy.as_ref()
    }

    pub fn dir_archive(&self) -> PathBuf {
        self.root_data.join(DIR_NAME_ARCHIVE)
    }

    pub fn dir_image(&self) -> PathBuf {
        self.root_data.join(DIR_NAME_IMAGE)
    }

    fn check(&self) -> Result<()> {
        let root_data = &self.root_data;
        if !root_data.exists() {
            fs::create_dir_all(root_data).map_err(|e| {
                anyhow!(
                    "Root data directory '{}' may not write: {}",
                    root_data.display(),
                    e
                )
            })?;
        }
        if let Some(root_deploy) = &self.root_deploy {
            if !root_deploy.exists() {
                fs::create_dir_all(root_deploy)?;
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

    fn save(&self) -> Result<()> {
        let config_path = config_path();
        fs::write(
            config_path,
            format!(
                "# Config File\n\n{}",
                toml::to_string_pretty(self)
                    .map_err(|e| anyhow!("Failed to serialize config: {}", e))?
            ),
        )
        .map_err(|e| anyhow!("Failed to write config file: {}", e))?;
        Ok(())
    }
}

impl From<RawConfig> for InternalConfig {
    fn from(value: RawConfig) -> Self {
        let root_data = value.root_data.unwrap_or(APP_ROOT.to_string());
        let root_data = PathBuf::from(root_data);
        let root_deploy = value.root_deploy.map(PathBuf::from);
        Self {
            root_data,
            root_deploy,
        }
    }
}
