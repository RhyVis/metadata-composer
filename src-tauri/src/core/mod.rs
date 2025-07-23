use std::{fs::File, sync::OnceLock};

use anyhow::{Result, anyhow};
use log::info;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::core::{
    config::{ConfigState, init_config},
    data::{init_data, state::DataState},
};

pub mod config;
pub mod data;
pub mod util;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn get_handle() -> AppHandle {
    APP_HANDLE
        .get()
        .cloned()
        .expect("Core not initialized, call init_core first")
}

pub fn get_handle_ref() -> &'static AppHandle {
    APP_HANDLE
        .get()
        .expect("Core not initialized, call init_core first")
}

/// Initializes the core components of the application.
pub fn init_core(app: &AppHandle) {
    APP_HANDLE
        .set(app.clone())
        .expect("Failed to set APP_HANDLE");
    if let Err(err) = init_core_internal(app) {
        app.dialog()
            .message(err.to_string().replace(':', ":\n"))
            .title("Core Initialization Error")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        app.exit(1);
    }
}

fn init_core_internal(app: &AppHandle) -> Result<()> {
    info!("Core initialization started");

    init_config(app)?;
    init_data(app)?;

    info!("Core initialized successfully");

    Ok(())
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

#[cfg(debug_assertions)]
const INIT_FLAG_NAME: &str = "init.dev.flag";

#[cfg(not(debug_assertions))]
const INIT_FLAG_NAME: &str = "init.flag";

fn check_init_flag(app: &AppHandle) -> Result<bool> {
    let mut path = app.path().app_config_dir()?;
    path.push(INIT_FLAG_NAME);
    Ok(path.exists())
}

fn create_init_flag(app: &AppHandle) -> Result<()> {
    let mut path = app.path().app_config_dir()?;
    path.push(INIT_FLAG_NAME);
    File::create(path).map_err(|e| anyhow!("Failed to create init flag: {}", e))?;
    Ok(())
}

pub trait AppStateExt {
    fn state_config(&self) -> State<'_, ConfigState>;

    fn state_data(&self) -> State<'_, DataState>;
}

impl AppStateExt for AppHandle {
    fn state_config(&self) -> State<'_, ConfigState> {
        self.state::<ConfigState>()
    }

    fn state_data(&self) -> State<'_, DataState> {
        self.state::<DataState>()
    }
}
