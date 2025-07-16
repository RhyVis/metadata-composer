pub mod config;
pub mod path_ext;

#[cfg(not(debug_assertions))]
pub const APP_ROOT: &str = ".";
#[cfg(debug_assertions)]
pub const APP_ROOT: &str = "./.dev";
