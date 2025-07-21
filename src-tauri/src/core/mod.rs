use crate::core::data::init_data;
use crate::core::util::config::init_config;
use crate::core::util::get_app_root_path;
use anyhow::{Result, anyhow};
use log::info;
use std::sync::OnceLock;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

pub mod data;
pub mod util;

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

fn init_core_internal(app: &AppHandle) -> Result<()> {
    info!("Core initialization started");

    let root_path = if !check_init_flag() {
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
        create_init_flag()?;
        path_chosen
    } else {
        None
    };

    init_config(
        if cfg!(debug_assertions) {
            get_app_root_path().to_owned()
        } else {
            app.path()
                .app_config_dir()
                .map_err(|e| anyhow!("Failed to get app config directory: {}", e))?
        },
        root_path,
    )?;
    init_data()?;

    APP_HANDLE
        .set(app.clone())
        .map_err(|_| anyhow::anyhow!("Core already initialized"))?;

    info!("Core initialized successfully");

    Ok(())
}

/// Initializes the core components of the application.
pub fn init_core(app_handle: &AppHandle) {
    if let Err(err) = init_core_internal(app_handle) {
        app_handle
            .dialog()
            .message(err.to_string())
            .title("Core Initialization Error")
            .kind(MessageDialogKind::Error)
            .blocking_show();
        app_handle.exit(666);
    }
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

const INIT_FLAG_NAME: &str = "INIT";

fn check_init_flag() -> bool {
    let root = get_app_root_path();
    let init_flag_path = root.join(INIT_FLAG_NAME);
    init_flag_path.exists()
}

fn create_init_flag() -> Result<()> {
    let root = get_app_root_path();
    let init_flag_path = root.join(INIT_FLAG_NAME);
    std::fs::File::create(init_flag_path)
        .map_err(|e| anyhow!("Failed to create init flag: {}", e))?;
    Ok(())
}
