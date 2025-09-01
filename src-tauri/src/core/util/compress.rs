use std::path::Path;

use anyhow::{Result, anyhow};
use encoding_rs::GBK;
use log::{debug, error, info};
use regex::Regex;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{ShellExt, process::CommandEvent};
use tokio::fs as tfs;

const EVENT_COMPRESSION_PROGRESS: &str = "compression_progress";
const EVENT_DECOMPRESSION_PROGRESS: &str = "decompression_progress";

pub async fn compress(
    app: &AppHandle,
    input_dir: impl AsRef<Path>,
    output_file: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    let shell = app.shell();
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

    if tfs::try_exists(&output_path).await? {
        tfs::remove_file(&output_path)
            .await
            .map_err(|e| anyhow!("Failed to remove existing output file: {e}"))?;
        info!("Removed existing output file: {}", output_path.display());
    }

    let mut command = command
        .arg("a")
        .arg(&output_path)
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

    let regex = Regex::new(r"^\s*(\d+)%\s+(\d+)\s+\+\s+(.+)$")
        .map_err(|e| anyhow!("Failed to compile regex for 7z output: {e}"))?;

    let mut exit_code = None;
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn compression command: {e}"))?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(out) => {
                let out_str = decode_out(&out);
                let out_str = out_str.trim();
                if !out_str.is_empty() {
                    if let Some((progress, file_count, file_path)) =
                        parse_7z_output(out_str, &regex)
                    {
                        info!(
                            "Compression progress: {progress:#?}% - {file_count} files - {file_path}"
                        );
                        let _ = app
                            .emit(
                                EVENT_COMPRESSION_PROGRESS,
                                (progress, file_count, file_path),
                            )
                            .inspect_err(|e| error!("Failed to send compression progress: {}", e));
                    } else {
                        debug!("Compression stdout|{out_str}");
                    }
                }
            },
            CommandEvent::Stderr(err) => {
                let err_str = decode_out(&err);
                let err_str = err_str.trim();
                if !err_str.is_empty() {
                    error!("Compression stderr|{err_str}");
                }
            },
            CommandEvent::Error(err) => {
                error!("Compression command error: {err}");
                return Err(anyhow!("Compression command error: {err}"));
            },
            CommandEvent::Terminated(termination) => {
                info!("Compression command terminated: {:?}", termination.code);
                exit_code = termination.code;
            },
            _ => {
                info!("Compression command event: {:?}", event);
            },
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
        if !tfs::try_exists(&output_path).await? {
            let mut check_path = output_path.clone();
            check_path.set_extension("7z");
            if tfs::try_exists(&check_path).await? {
                tfs::rename(&check_path, &output_path).await?;
            } else {
                return Err(anyhow!(
                    "Output file not found after compression: {}",
                    output_path.display()
                ));
            }
        }

        info!("Compression completed successfully.");

        Ok(())
    }
}

pub async fn decompress(
    app: &AppHandle,
    input_file: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    password: Option<&str>,
) -> Result<()> {
    let shell = app.shell();
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

    let regex = Regex::new(r"^\s*(\d+)%\s+(\d+)\s+-\s+(.+)$")
        .map_err(|e| anyhow!("Failed to compile regex for 7z output: {e}"))?;

    let mut exit_code = None;
    let (mut rx, _child) = command
        .spawn()
        .map_err(|e| anyhow!("Failed to spawn decompression command: {e}"))?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(out) => {
                let out_str = decode_out(&out);
                let out_str = out_str.trim();
                if !out_str.is_empty() {
                    if let Some((progress, file_count, file_path)) =
                        parse_7z_output(out_str, &regex)
                    {
                        info!(
                            "Decompression progress: {progress:#?}% - {file_count} files - {file_path}"
                        );
                        let _ = app
                            .emit(
                                EVENT_DECOMPRESSION_PROGRESS,
                                (progress, file_count, file_path),
                            )
                            .inspect_err(|e| {
                                error!("Failed to send decompression progress: {}", e)
                            });
                    } else {
                        debug!("Decompression stdout|{out_str}");
                    }
                }
            },
            CommandEvent::Stderr(err) => {
                let err_str = decode_out(&err);
                let err_str = err_str.trim();
                if !err_str.is_empty() {
                    error!("Decompress stderr|{err_str}");
                }
            },
            CommandEvent::Error(err) => {
                error!("Decompression command error: {err}");
                return Err(anyhow!("Decompression command error: {err}"));
            },
            CommandEvent::Terminated(termination) => {
                info!("Decompression command terminated: {:?}", termination.code);
                exit_code = termination.code;
            },
            _ => {
                info!("Decompression command event: {:?}", event);
            },
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
        let (cow, ..) = GBK.decode(out);
        cow.to_string()
    };

    #[cfg(not(target_os = "windows"))]
    let out = String::from_utf8_lossy(out);

    out
}

fn parse_7z_output<'a>(out: &'a str, regex: &Regex) -> Option<(u32, u32, &'a str)> {
    if let Some(captures) = regex.captures(out) {
        let progress = captures.get(1)?.as_str().parse::<u32>().ok()?;
        let file_count = captures.get(2)?.as_str().parse::<u32>().ok()?;
        let file_name = captures.get(3)?.as_str();

        Some((progress, file_count, file_name))
    } else {
        None
    }
}
