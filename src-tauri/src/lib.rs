use crate::command::*;
use crate::core::init_core;
use tauri::{generate_context, generate_handler};

mod api;
mod command;
mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .setup(|app| {
            let handle = app.handle();

            if cfg!(debug_assertions) {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            } else {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Error)
                        .build(),
                )?;
            }

            init_core(handle)?;

            Ok(())
        })
        .invoke_handler(generate_handler![
            metadata_update,
            metadata_get_all,
            metadata_get,
            metadata_delete,
            metadata_collection_list,
            metadata_deploy,
            metadata_deploy_off,
            metadata_export,
            metadata_import,
            util_process_img,
            util_clear_unused_images,
            util_dark_state,
            path_resolve_img,
            config_get,
            config_update
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
