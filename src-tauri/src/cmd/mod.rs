pub mod append;

use std::path::{Path, PathBuf};

use serde_json::Value;
use tauri::{AppHandle, Manager, State, command};
use tauri_plugin_opener::open_path;
use tauri_plugin_pinia::ManagerExt;

use crate::{
    api::dl_site::{DLContentFetch, DLFetchInfo},
    cmd::append::{DLFetchArg, DeployArg},
    core::{
        Language, StringResult,
        config::{AppConfig, ConfigState},
        data::{
            library,
            metadata::{Metadata, MetadataOption},
            state::DataState,
        },
        util,
    },
};

type CommandResult<T> = Result<T, String>;

#[command]
pub async fn metadata_update(
    opt: MetadataOption,
    data: State<'_, DataState>,
) -> CommandResult<Option<String>> {
    library::metadata_update(opt, data).await.string_result()
}

#[command]
pub async fn metadata_get_all(data: State<'_, DataState>) -> CommandResult<Vec<Metadata>> {
    library::metadata_get_all(data).await.string_result()
}

#[command]
pub async fn metadata_get(
    key: String,
    data: State<'_, DataState>,
) -> CommandResult<Option<Metadata>> {
    library::metadata_get(key, data).await.string_result()
}

#[command]
pub async fn metadata_delete(key: String, data: State<'_, DataState>) -> CommandResult<()> {
    library::metadata_delete(key, data).await.string_result()
}

#[command]
pub async fn metadata_deploy(key: String, arg: DeployArg, app: AppHandle) -> CommandResult<()> {
    library::metadata_deploy(key, arg, app)
        .await
        .string_result()
}

#[command]
pub async fn metadata_deploy_off(key: String, data: State<'_, DataState>) -> CommandResult<()> {
    library::metadata_deploy_off(key, data)
        .await
        .string_result()
}

#[command]
pub fn metadata_collection_cache(data: State<'_, DataState>) -> CommandResult<Vec<String>> {
    library::collection_cache_get(data).string_result()
}

#[command]
pub fn metadata_deployment_cache(data: State<'_, DataState>) -> CommandResult<Vec<String>> {
    library::deployment_cache_get(data).string_result()
}

#[command]
pub async fn metadata_export(app: AppHandle) -> CommandResult<()> {
    library::export_library(app).await.string_result()
}

#[command]
pub async fn metadata_import(app: AppHandle) -> CommandResult<()> {
    library::import_library(app).await.string_result()
}

#[command]
pub async fn util_process_img_file(source: String, app: AppHandle) -> CommandResult<String> {
    util::img::process_image_file(source, app)
        .await
        .string_result()
}

#[command]
pub async fn util_process_img_web(url: String, app: AppHandle) -> CommandResult<String> {
    util::img::process_image_web(&url, app)
        .await
        .string_result()
}

#[command]
pub async fn util_process_img_bytes(
    data: (Vec<u8>, u32, u32),
    app: AppHandle,
) -> CommandResult<String> {
    util::img::process_image_bytes(data, app)
        .await
        .string_result()
}

#[command]
pub async fn util_clear_unused_images(app: AppHandle) -> CommandResult<u32> {
    library::clear_unused_images(app).await.string_result()
}

#[command]
pub async fn util_dl_fetch_info(arg: DLFetchArg) -> CommandResult<DLFetchInfo> {
    arg.content_type
        .fetch_info(arg.id.as_str(), &Language::ZhCn)
        .await
        .string_result()
}

#[command]
pub fn util_dark_state(app: AppHandle) -> bool {
    app.pinia()
        .try_get_or::<bool>("global", "isDarkMode", false)
}

#[command]
pub fn open_config_dir(app: AppHandle) -> CommandResult<()> {
    open_path(
        app.path()
            .app_config_dir()
            .map_err(|e| format!("Failed to get app config directory: {}", e))?,
        None::<&str>,
    )
    .string_result()
}

#[command]
pub fn open_log_dir(app: AppHandle) -> CommandResult<()> {
    open_path(
        app.path()
            .app_log_dir()
            .map_err(|e| format!("Failed to get app log directory: {}", e))?,
        None::<&str>,
    )
    .string_result()
}

#[command]
pub async fn path_resolve_img(
    hash: String,
    config: State<'_, ConfigState>,
) -> CommandResult<String> {
    let mut base = config.get().dir_image();
    base.push(format!("{hash}.png"));
    let abs = tokio::fs::canonicalize(base).await.string_result()?;
    if abs.exists() {
        Ok(abs.to_string_lossy().to_string())
    } else {
        Err(format!("Image not found: {}", abs.display()))
    }
}

#[command]
pub async fn path_resolve_archive(
    path: String,
    config: State<'_, ConfigState>,
) -> CommandResult<PathBuf> {
    let mut base = config.get().dir_image();
    base.push(Path::new(&path));
    tokio::fs::canonicalize(base).await.string_result()
}

#[command]
pub fn config_get(config: State<'_, ConfigState>) -> AppConfig {
    config.get()
}

#[command]
pub fn config_update(
    name: String,
    value: Value,
    config: State<'_, ConfigState>,
) -> CommandResult<()> {
    config.update_field(&name, value).string_result()
}
