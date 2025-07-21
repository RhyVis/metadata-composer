use crate::core::data::init_data;
use crate::core::util::config::init_config;
use log::info;
use std::sync::OnceLock;
use tauri::AppHandle;

pub mod data;
pub mod util;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

/// Initializes the core components of the application.
pub fn init_core(app_handle: &AppHandle) -> anyhow::Result<()> {
    info!("Core initialization started");

    init_config()?;
    init_data()?;

    APP_HANDLE
        .set(app_handle.clone())
        .map_err(|_| anyhow::anyhow!("Core already initialized"))?;

    info!("Core initialized successfully");
    Ok(())
}

pub fn get_handle() -> AppHandle {
    APP_HANDLE
        .get()
        .cloned()
        .expect("Core not initialized, call init_core first")
}

pub trait StringResult<T, E>
where
    E: std::fmt::Display,
{
    fn string_result(self) -> Result<T, String>;
}

impl<T, E> StringResult<T, E> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn string_result(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

/// This ... That ...
pub enum Whether<This, That> {
    This(This),
    That(That),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Language {
    EnUs,
    ZhCn,
    JaJp,
}
