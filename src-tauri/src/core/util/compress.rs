use crate::core::get_handle;
use anyhow::{Result, anyhow};
use encoding_rs::GBK;
use log::{debug, error, info};
use regex::Regex;
use std::path::Path;
use tauri::Emitter;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use tokio::fs as tfs;

const EVENT_COMPRESSION_PROGRESS: &str = "compression_progress";
const EVENT_DECOMPRESSION_PROGRESS: &str = "decompression_progress";

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
        "Compressing: input_dir: {}, output_file: {}, password: {:?}",
        input_dir.as_ref().display(),
        output_file.as_ref().display(),
        password
    );

    let input_path = input_dir.as_ref().to_owned();
    let output_path = output_file.as_ref().to_owned();
    let password = password.map(|s| s.to_owned());

    if output_path.exists() {
        tfs::remove_file(&output_path)
            .await
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
        .arg("-r")
        .arg("-bsp1");
    if let Some(pwd) = password {
        command = command.arg(format!("-p{pwd}")).arg("-mhe");
    }

    let progress_regex = Regex::new(r"^\s*(\d+)%")?;
    let mut last_progress = 0u32;

    let mut exit_code = None;
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn compression command: {e}"))?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(out) => {
                let out_str = decode_out(&out);
                if !out_str.is_empty() {
                    progress_regex
                        .captures(&out_str)
                        .and_then(|captures| captures.get(1))
                        .and_then(|matching| matching.as_str().parse::<u32>().ok())
                        .map(|progress| {
                            if progress != last_progress {
                                last_progress = progress;
                                info!("Compression progress: {progress:#?}%");
                                let _ = handle
                                    .emit(EVENT_COMPRESSION_PROGRESS, progress)
                                    .inspect_err(|e| {
                                        error!("Failed to send compression progress: {}", e)
                                    });
                            }
                        })
                        .unwrap_or_else(|| {
                            debug!("Compression stdout|{out_str}");
                        })
                }
            }
            CommandEvent::Stderr(err) => {
                let err_str = decode_out(&err);
                if !err_str.is_empty() {
                    error!("Compression stderr|{err_str}");
                }
            }
            CommandEvent::Error(err) => {
                error!("Compression command error: {err}");
                return Err(anyhow!("Compression command error: {err}"));
            }
            CommandEvent::Terminated(termination) => {
                info!("Compression command terminated: {:?}", termination.code);
                exit_code = termination.code;
            }
            _ => {
                info!("Compression command event: {:?}", event);
            }
        }
    }

    if exit_code != Some(0) {
        let err_msg = format!(
            "Compression command failed with exit code: {:?}",
            exit_code.unwrap_or(-1)
        );
        error!("{}", err_msg);
        Err(anyhow!(err_msg))
    } else {
        info!("Compression completed successfully.");
        Ok(())
    }
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
        .arg("-y")
        .arg("-bsp1");
    if let Some(pwd) = password {
        command = command.arg(format!("-p{pwd}"));
    }

    let progress_regex = Regex::new(r"^\s*(\d+)%")?;
    let mut last_progress = 0u32;

    let mut exit_code = None;
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn decompression command: {e}"))?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(out) => {
                let out_str = decode_out(&out);
                if !out_str.is_empty() {
                    progress_regex
                        .captures(&out_str)
                        .and_then(|captures| captures.get(1))
                        .and_then(|matching| matching.as_str().parse::<u32>().ok())
                        .map(|progress| {
                            if progress != last_progress {
                                last_progress = progress;
                                info!("Decompression progress: {progress:#?}%");
                                let _ = handle
                                    .emit(EVENT_DECOMPRESSION_PROGRESS, progress)
                                    .inspect_err(|e| {
                                        error!("Failed to send decompression progress: {}", e)
                                    });
                            }
                        })
                        .unwrap_or_else(|| {
                            debug!("Decompression stdout|{out_str}");
                        })
                }
            }
            CommandEvent::Stderr(err) => {
                let err_str = decode_out(&err);
                if !err_str.is_empty() {
                    error!("Decompress stderr|{err_str}");
                }
            }
            CommandEvent::Error(err) => {
                error!("Decompression command error: {err}");
                return Err(anyhow!("Decompression command error: {err}"));
            }
            CommandEvent::Terminated(termination) => {
                info!("Decompression command terminated: {:?}", termination.code);
                exit_code = termination.code;
            }
            _ => {
                info!("Decompression command event: {:?}", event);
            }
        }
    }

    if exit_code != Some(0) {
        let err_msg = format!(
            "Decompression command failed with exit code: {:?}",
            exit_code.unwrap_or(-1)
        );
        error!("{}", err_msg);
        Err(anyhow!(err_msg))
    } else {
        info!("Decompression completed successfully.");
        Ok(())
    }
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
