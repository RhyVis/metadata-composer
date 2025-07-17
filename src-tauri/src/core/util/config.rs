use crate::core::util::APP_ROOT;
use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

const CONFIG_FILE_NAME: &str = "config.toml";

const DIR_NAME_ARCHIVE: &str = "archive";

static CONFIG: OnceLock<InternalConfig> = OnceLock::new();

pub fn init_config() -> Result<()> {
    let config_path = PathBuf::from(APP_ROOT).join(CONFIG_FILE_NAME);
    info!("Trying to load config from: {}", config_path.display());
    let config: InternalConfig = if !config_path.exists() {
        warn!("Config file does not exist");
        ExternalConfig::default()
    } else {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
        toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?
    }
    .into();

    config.check()?;

    CONFIG
        .set(config)
        .map_err(|_| anyhow::anyhow!("Config already initialized"))?;

    Ok(())
}

pub fn get_config() -> Result<&'static InternalConfig> {
    CONFIG
        .get()
        .ok_or_else(|| anyhow::anyhow!("Config not initialized"))
}

pub fn get_config_copy() -> Result<InternalConfig> {
    get_config().map(|config| config.clone())
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct ExternalConfig {
    root: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InternalConfig {
    root: PathBuf,
}

impl InternalConfig {
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn dir_archive(&self) -> PathBuf {
        self.root.join(DIR_NAME_ARCHIVE)
    }

    fn check(&self) -> Result<()> {
        let root = self.root();
        if !root.exists() {
            fs::create_dir_all(root)?;
        }
        let dir_archive = self.dir_archive();
        if !dir_archive.exists() {
            fs::create_dir_all(&dir_archive)?;
        }

        Ok(())
    }
}

impl From<ExternalConfig> for InternalConfig {
    fn from(value: ExternalConfig) -> Self {
        let root = value.root.unwrap_or(APP_ROOT.to_string());
        let root = PathBuf::from(root);
        Self { root }
    }
}
