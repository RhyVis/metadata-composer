use const_format::formatc;

pub mod compress;
pub mod img;
pub mod path_ext;

#[cfg(not(debug_assertions))]
pub const APP_ROOT: &str = ".";
#[cfg(debug_assertions)]
pub const APP_ROOT: &str = "./.dev";

pub const DEV_LOG_DIR: &str = formatc!("{APP_ROOT}/logs");
