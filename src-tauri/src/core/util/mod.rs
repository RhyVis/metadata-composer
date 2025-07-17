use std::process::Command;

pub mod compress;
pub mod config;
pub mod path_ext;

#[cfg(not(debug_assertions))]
pub const APP_ROOT: &str = ".";
#[cfg(debug_assertions)]
pub const APP_ROOT: &str = "./.dev";

pub fn create_hidden_command(cmd: &str) -> Command {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let mut command = Command::new(cmd);
        command.creation_flags(CREATE_NO_WINDOW);
        command
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new(cmd)
    }
}
