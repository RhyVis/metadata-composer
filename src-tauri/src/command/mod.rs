use crate::core::StringResult;
use crate::core::data::library;
use crate::core::data::metadata::{Metadata, MetadataOption};
use tauri::command;

type CommandResult<T> = Result<T, String>;

#[command]
pub fn lib_update(opt: MetadataOption) -> CommandResult<Option<String>> {
    library::lib_update(opt).string_result()
}

#[command]
pub fn lib_get_all() -> CommandResult<Vec<Metadata>> {
    library::lib_get_all().string_result()
}

#[command]
pub fn lib_get(key: String) -> CommandResult<Option<Metadata>> {
    library::lib_get(&key).string_result()
}
