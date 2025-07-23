use std::sync::RwLock;

use anyhow::Result;
use serde_json::Value;

use super::def::AppConfig;

pub struct ConfigState(RwLock<AppConfig>);

impl ConfigState {
    pub(super) fn new(config: AppConfig) -> Self {
        Self(RwLock::new(config))
    }

    pub fn get(&self) -> AppConfig {
        self.0.read().expect("Failed to acquire read lock").clone()
    }

    fn write(&self, f: impl FnOnce(&mut AppConfig) -> Result<()>) -> Result<()> {
        let mut config = self.0.write().expect("Failed to acquire write lock");
        f(&mut *config)?;
        Ok(())
    }

    pub fn update_field(&self, name: &str, value: Value) -> Result<()> {
        self.write(|config| config.update_field(name, value))
    }
}
