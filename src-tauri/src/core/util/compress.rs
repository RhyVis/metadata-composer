use crate::core::get_handle;
use anyhow::{Result, anyhow};
use encoding_rs::GBK;
use log::{error, info};
use std::fs;
use std::path::Path;
use tauri_plugin_shell::ShellExt;

pub async fn compress(
    input_dir: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    let handle = get_handle();
    let shell = handle.shell();
    let command = shell
        .sidecar("7z")
        .map_err(|e| anyhow!("Failed to get 7z sidecar: {e}"))?;

    info!(
        "Compressing with 7z: input_dir: {}, output_file: {}, password: {:?}",
        input_dir.as_ref().display(),
        output_file.as_ref().display(),
        password
    );

    let input_path = input_dir.as_ref().to_owned();
    let output_path = output_file.as_ref().to_owned();
    let password = password.map(|s| s.to_owned());

    if output_path.exists() {
        fs::remove_file(&output_path)
            .map_err(|e| anyhow!("Failed to remove existing output file: {e}"))?;
        info!("Removed existing output file: {}", output_path.display());
    }

    let mut command = command
        .arg("a")
        .arg(output_path)
        .arg(input_path.join("*"))
        .arg("-t7z")
        .arg("-mx9")
        .arg("-ms4g")
        .arg("-md64m")
        .arg("-mmt")
        .arg("-r");
    if let Some(pwd) = password {
        command = command.arg(format!("-p{pwd}")).arg("-mhe");
    }

    command
        .output()
        .await
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

pub async fn decompress(
    input_file: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    let handle = get_handle();
    let shell = handle.shell();
    let command = shell
        .sidecar("7z")
        .map_err(|e| anyhow!("Failed to get 7z sidecar: {e}"))?;

    info!(
        "Decompressing with 7z: input_file: {}, output_dir: {}, password: {:?}",
        input_file.as_ref().display(),
        output_dir.as_ref().display(),
        password
    );

    let input_path = input_file.as_ref().to_owned();
    let output_path = output_dir.as_ref().to_owned();
    let password = password.map(|s| s.to_owned());

    let mut command = command
        .arg("x")
        .arg(input_path)
        .arg(format!("-o{}", output_path.display()))
        .arg("-aoa")
        .arg("-y");

    if let Some(pwd) = password {
        command = command.arg(format!("-p{pwd}"));
    }

    command
        .output()
        .await
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
