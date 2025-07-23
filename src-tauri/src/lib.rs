use std::path::PathBuf;

use tauri::Manager;

use crate::{
    api::init_api,
    core::{init_core, util::DEV_LOG_DIR},
    init::BuilderExt,
};

pub mod api;
pub mod cmd;
pub mod core;
pub mod init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app
                .get_webview_window("main")
                .expect("Failed to get main window");
            let _ = window.show();
            let _ = window.set_focus();
        }))
        .setup(|app| {
            let handle = app.handle();

            if cfg!(debug_assertions) {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Debug)
                        .target(tauri_plugin_log::Target::new(
                            tauri_plugin_log::TargetKind::Folder {
                                path: PathBuf::from(DEV_LOG_DIR),
                                file_name: Some(String::from("Composer")),
                            },
                        ))
                        .build(),
                )?;
            } else {
                handle.plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .target(tauri_plugin_log::Target::new(
                            tauri_plugin_log::TargetKind::LogDir {
                                file_name: Some(String::from("Composer")),
                            },
                        ))
                        .max_file_size(50_000)
                        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
                        .build(),
                )?;
            }

            init_core(handle);
            init_api()?;

            Ok(())
        })
        .register_invoke_handler()
        .run_with_context();
}
