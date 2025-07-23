use tauri::{Builder, Wry, generate_handler};

use crate::cmd::*;

pub fn register_invoke_handler(builder: Builder<Wry>) -> Builder<Wry> {
    builder.invoke_handler(generate_handler![
        metadata_update,
        metadata_get_all,
        metadata_get,
        metadata_delete,
        metadata_deploy,
        metadata_deploy_off,
        metadata_collection_cache,
        metadata_deployment_cache,
        metadata_export,
        metadata_import,
        util_process_img_file,
        util_process_img_web,
        util_process_img_bytes,
        util_clear_unused_images,
        util_dl_fetch_info,
        util_dark_state,
        open_config_dir,
        open_log_dir,
        path_resolve_img,
        path_resolve_archive,
        config_get,
        config_update
    ])
}
