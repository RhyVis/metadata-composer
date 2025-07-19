use const_format::formatc;
use std::process::Command;

pub mod compress;
pub mod config;
pub mod img;
pub mod path_ext;

#[cfg(not(debug_assertions))]
pub const APP_ROOT: &str = ".";
#[cfg(debug_assertions)]
pub const APP_ROOT: &str = "./.dev";

pub const APP_LOG_DIR: &str = formatc!("{APP_ROOT}/logs");

pub fn create_hidden_command(cmd: &str) -> Command {
    let mut command = Command::new(cmd);

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        command.creation_flags(CREATE_NO_WINDOW);
    }

    command
}
