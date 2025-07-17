use crate::core::data::metadata::{Metadata, MetadataOption};
use crate::core::util::config::get_config;
use crate::core::{StringResult, data, util};
use tauri::command;

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
pub fn util_process_img(source: String) -> CommandResult<String> {
    util::img::process_image(source).string_result()
}

#[command]
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
