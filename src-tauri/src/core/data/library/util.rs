use std::collections::HashSet;

use anyhow::anyhow;
use log::{debug, info, warn};
use tauri::AppHandle;
use tokio::fs as tfs;

use crate::core::{AppStateExt, data::library::metadata_get_all};

pub async fn clear_unused_images(app: AppHandle) -> anyhow::Result<u32> {
    let all_used_images = metadata_get_all(app.state_data())
        .await?
        .iter()
        .map(|data| data.image.clone())
        .flatten()
        .collect::<HashSet<_>>();
    let dir_image = app.state_config().get().dir_image();
    if !dir_image.exists() || !dir_image.is_dir() {
        return Err(anyhow!(
            "Image directory does not exist or is not a directory: {}",
            dir_image.display()
        ));
    }

    let mut entries = tfs::read_dir(&dir_image).await?;
    let mut removed_count = 0u32;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "png") {
            if let Some(file_name) = path
                .file_stem()
                .and_then(|s| Some(s.to_string_lossy().to_string()))
            {
                if !all_used_images.contains(&file_name) {
                    let _ = tfs::remove_file(&path)
                        .await
                        .inspect(|_| {
                            debug!("Removed unused image: {}", path.display());
                            removed_count += 1;
                        })
                        .inspect_err(|e| {
                            warn!("Failed to remove unused image '{}': {}", path.display(), e)
                        });
                }
            }
        }
    }

    if removed_count > 0 {
        info!(
            "Removed {} unused images from directory: {}",
            removed_count,
            dir_image.display()
        );
    } else {
        info!(
            "No unused images found in directory: {}",
            dir_image.display()
        );
    }
    Ok(removed_count)
}
