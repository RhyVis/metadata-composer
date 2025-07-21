use const_format::formatc;
use std::path::Path;

pub mod compress;
pub mod config;
pub mod img;
pub mod path_ext;

#[cfg(not(debug_assertions))]
pub const APP_ROOT: &str = ".";
#[cfg(debug_assertions)]
pub const APP_ROOT: &str = "./.dev";

pub const APP_LOG_DIR: &str = formatc!("{APP_ROOT}/logs");

pub fn get_app_root_path() -> &'static Path {
    Path::new(APP_ROOT)
}
