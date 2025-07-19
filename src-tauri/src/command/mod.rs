pub mod append;

use crate::api::dl_site::{DLContentFetch, DLFetchInfo};
use crate::command::append::{DLFetchArg, DeployArg};
use crate::core::data::metadata::{Metadata, MetadataOption};
use crate::core::util::config::{InternalConfig, get_config, get_config_copy, update_config_field};
use crate::core::{Language, StringResult, data, util};
use tauri::{AppHandle, command};
use tauri_plugin_pinia::ManagerExt;

type CommandResult<T> = Result<T, String>;

#[command]
pub fn metadata_update(opt: MetadataOption) -> CommandResult<Option<String>> {
    data::metadata_update(opt).string_result()
}

#[command]
pub fn metadata_get_all() -> CommandResult<Vec<Metadata>> {
    data::metadata_get_all().string_result()
}

#[command]
pub fn metadata_get(key: String) -> CommandResult<Option<Metadata>> {
    data::metadata_get(&key).string_result()
}

#[command]
pub fn metadata_delete(key: String) -> CommandResult<()> {
    data::metadata_delete(&key).string_result()
}

#[command]
pub fn metadata_collection_list() -> CommandResult<Vec<String>> {
    data::metadata_collection_list().string_result()
}

#[command]
pub fn metadata_deploy(key: String, arg: DeployArg) -> CommandResult<()> {
    data::metadata_deploy(key.as_str(), arg).string_result()
}

#[command]
pub fn metadata_deploy_off(key: String) -> CommandResult<()> {
    data::metadata_deploy_off(&key).string_result()
}

#[command]
pub fn metadata_export() -> CommandResult<()> {
    data::export_library().string_result()
}

#[command]
pub fn metadata_import() -> CommandResult<()> {
    data::import_library().string_result()
}

#[command]
pub fn util_process_img(source: String) -> CommandResult<String> {
    util::img::process_image(source).string_result()
}

#[command]
pub fn util_clear_unused_images() -> CommandResult<u32> {
    data::clear_unused_images().string_result()
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

#[command(async)]
pub fn path_resolve_img(hash: String) -> CommandResult<String> {
    let mut base = get_config().string_result()?.dir_image();
    base.push(format!("{hash}.png"));
    let abs = base.canonicalize().string_result()?;
    if abs.exists() {
        Ok(abs.to_string_lossy().to_string())
    } else {
        Err(format!("Image not found: {}", abs.display()))
    }
}

#[command]
pub fn config_get() -> CommandResult<InternalConfig> {
    get_config_copy().string_result()
}

#[command]
pub fn config_update(name: String, value: Option<String>) -> CommandResult<()> {
    update_config_field(name, value).string_result()
}
