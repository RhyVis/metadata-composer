use anyhow::{Result, anyhow};
use log::info;
use tauri::{AppHandle, Manager};

mod def;
mod state;

pub use self::{def::AppConfig, state::ConfigState};

pub fn init_config(app: &AppHandle) -> Result<()> {
    if app.manage(ConfigState::new(AppConfig::load(app)?)) {
        info!("AppConfig initialized successfully.");
        Ok(())
    } else {
        Err(anyhow!("Failed to initialize AppConfig."))
    }
}
