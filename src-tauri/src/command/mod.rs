use crate::core::StringResult;
use crate::core::data::library;
use crate::core::data::metadata::{Metadata, MetadataOption};
use tauri::command;

type CommandResult<T> = Result<T, String>;

#[command]
pub fn metadata_update(opt: MetadataOption) -> CommandResult<Option<String>> {
    library::metadata_update(opt).string_result()
}

#[command]
pub fn metadata_get_all() -> CommandResult<Vec<Metadata>> {
    library::metadata_get_all().string_result()
}

#[command]
pub fn metadata_get(key: String) -> CommandResult<Option<Metadata>> {
    library::metadata_get(&key).string_result()
}

#[command]
pub fn metadata_delete(key: String) -> CommandResult<()> {
    library::metadata_delete(&key).string_result()
}

#[command]
pub fn metadata_collection_list() -> CommandResult<Vec<String>> {
    library::metadata_collection_list().string_result()
}
