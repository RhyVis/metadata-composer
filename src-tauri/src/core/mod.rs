use std::{fs::File, sync::OnceLock};

use anyhow::{Result, anyhow};
use log::info;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::core::{data::init_data, util::config::init_config};

pub mod data;
pub mod util;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

pub fn get_handle() -> AppHandle {
    APP_HANDLE
        .get()
        .cloned()
        .expect("Core not initialized, call init_core first")
}

/// Initializes the core components of the application.
pub fn init_core(app_handle: &AppHandle) {
    if let Err(err) = init_core_internal(app_handle) {
        app_handle
            .dialog()
            .message(err.to_string().replace(':', ":\n"))
            .title("Core Initialization Error")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        app_handle.exit(666);
    }
}

fn init_core_internal(app: &AppHandle) -> Result<()> {
    info!("Core initialization started");

    let alt_root_path =
        if !check_init_flag(app).map_err(|e| anyhow!("Failed to check init flag: {}", e))? {
            app.dialog()
                .message(
                    "Seems that the application is running for the first time.\n\
                You'll need to specify the root directory to store data.",
                )
                .title("Initialization Required")
                .blocking_show();
            let path_chosen = app
                .dialog()
                .file()
                .set_title("Root Directory Selection")
                .blocking_pick_folder()
                .and_then(|path| path.into_path().ok());
            create_init_flag(app)?;
            path_chosen
        } else {
            None
        };

    init_config(
        app.path()
            .app_config_dir()
            .map_err(|e| anyhow!("Failed to get app config directory: {}", e))?,
        alt_root_path,
    )?;
    init_data()?;

    APP_HANDLE
        .set(app.clone())
        .map_err(|_| anyhow::anyhow!("Core already initialized"))?;

    info!("Core initialized successfully");

    Ok(())
}

pub trait StringResult<T, E>
where E: std::fmt::Display
{
    fn string_result(self) -> Result<T, String>;
}

impl<T, E> StringResult<T, E> for Result<T, E>
where E: std::fmt::Display
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
