use crate::core::util::create_hidden_command;
use anyhow::{Result, anyhow};
use encoding_rs::GBK;
use log::{error, info};
use std::path::Path;
use std::process::Command;

fn exist_7z_exe() -> bool {
    match Command::new("7z").arg("--help").output() {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

pub fn compress(
    input_dir: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    password: Option<&str>,
    compression_level: Option<u32>,
) -> Result<()> {
    if !exist_7z_exe() {
        return Err(anyhow!("7z not found in path"));
    }

    info!(
        "Compressing with 7z: input_dir: {}, output_file: {}, password: {:?}, compression_level: {:?}",
        input_dir.as_ref().display(),
        output_file.as_ref().display(),
        password,
        compression_level
    );

    let input_path = input_dir.as_ref().to_owned();
    let output_path = output_file.as_ref().to_owned();
    let password = password.map(|s| s.to_owned());
    let compression_level = match compression_level {
        Some(level) => {
            if level <= 9 {
                level
            } else {
                9
            }
        }
        None => 9,
    };

    let mut command = create_hidden_command("7z");

    command
        .arg("a")
        .arg("-t7z")
        .arg(output_path)
        .arg(input_path.join("*"))
        .arg(format!("-mx={compression_level}"))
        .arg("-ms")
        .arg("-mmt")
        .arg("-r");

    if let Some(pwd) = password {
        command.arg(format!("-p{pwd}")).arg("-mhe=on");
    }

    command
        .output()
        .map_err(|err| anyhow!("Compression command exec failed, {err}"))
        .and_then(|out| {
            if out.status.success() {
                info!("Compression completed successfully.");
                Ok(())
            } else {
                let err_msg = decode_out(&out.stderr);
                error!("Compression failed: {err_msg}");
                Err(anyhow!("Compression failed: {err_msg}"))
            }
        })
}

pub fn decompress(
    input_file: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    if !exist_7z_exe() {
        return Err(anyhow!("7z not found in path"));
    }

    info!(
        "Decompressing with 7z: input_file: {}, output_dir: {}, password: {:?}",
        input_file.as_ref().display(),
        output_dir.as_ref().display(),
        password
    );

    let input_path = input_file.as_ref().to_owned();
    let output_path = output_dir.as_ref().to_owned();
    let password = password.map(|s| s.to_owned());

    let mut command = create_hidden_command("7z");

    command
        .arg("x")
        .arg(input_path)
        .arg(format!("-o{}", output_path.display()))
        .arg("-aoa");

    if let Some(pwd) = password {
        command.arg(format!("-p{pwd}"));
    }

    command
        .output()
        .map_err(|e| anyhow!("Decompression command failed: {e}"))
        .and_then(|out| {
            if out.status.success() {
                info!("Decompression completed successfully.");
                Ok(())
            } else {
                let err_msg = decode_out(&out.stderr);
                error!("Decompression failed: {err_msg}");
                Err(anyhow!("Decompression failed: {err_msg}"))
            }
        })
}

fn decode_out(out: &[u8]) -> String {
    #[cfg(target_os = "windows")]
    let out = {
        let (cow, _, _) = GBK.decode(out);
        cow.to_string()
    };

    #[cfg(not(target_os = "windows"))]
    let out = String::from_utf8_lossy(out);

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_7z() {
        if exist_7z_exe() {
            println!("7z found in path");
        } else {
            println!("7z not found in path");
        }
    }
}
