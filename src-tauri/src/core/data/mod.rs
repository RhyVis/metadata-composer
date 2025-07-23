use anyhow::Result;
use tauri::AppHandle;

use crate::core::data::library::init_library;

pub mod library;
pub mod metadata;
pub mod state;

pub fn init_data(app: &AppHandle) -> Result<()> {
    init_library(app)?;
    Ok(())
}
