use std::collections::HashSet;

use anyhow::{Error, Result, anyhow};
use log::{debug, info, warn};
use redb::ReadableTable;
use tauri::{AppHandle, async_runtime};
use tokio::fs as tfs;

use crate::core::{
    AppStateExt,
    data::{
        library::{TABLE_METADATA, metadata_get_all},
        metadata::Metadata,
    },
};

pub async fn clear_unused_images(app: AppHandle) -> Result<u32> {
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

pub async fn clear_unused_deploy_dirs(app: AppHandle) -> Result<u32> {
    let config = app.state_config().get();
    let Some(path) = config.path_deploy() else {
        return Ok(0);
    };
    if !tfs::try_exists(path).await? {
        return Ok(0);
    }

    let mut dirs = tfs::read_dir(path).await?;
    let mut count = 0u32;
    while let Some(entry) = dirs.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            if let None = tfs::read_dir(&path).await?.next_entry().await? {
                match tfs::remove_dir_all(&path).await {
                    Ok(_) => {
                        count += 1;
                        debug!("Removed unused deploy directory: {}", path.display())
                    },
                    Err(e) => warn!(
                        "Failed to remove deploy directory '{}': {}",
                        path.display(),
                        e
                    ),
                }
            }
        }
    }

    Ok(count)
}

pub async fn recalculate_archive_size(app: AppHandle) -> Result<()> {
    let data = app.state_data();
    let db = data.database();
    let db_read = db.clone();

    let edited_metadata = async_runtime::spawn_blocking(move || {
        let read = db_read.begin_read()?;
        let table = read.open_table(TABLE_METADATA)?;
        let mut new_metadata = Vec::new();
        for entry in table.iter()?.filter_map(Result::ok) {
            let (key, value) = (entry.0.value().to_owned(), entry.1.value());
            let mut metadata = bson::from_slice::<Metadata>(&value)?;
            let _ = metadata
                .archive_info
                .update_size_blocking()
                .inspect_err(|e| warn!("Failed to update archive info size of {}: {}", key, e));
            new_metadata.push((key.clone(), bson::to_vec(&metadata)?));
        }
        Ok::<Vec<(String, Vec<u8>)>, Error>(new_metadata)
    })
    .await??;

    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            for (key, value) in edited_metadata {
                table.insert(&*key, value)?;
                debug!("Updated archive size for metadata ID: {}", key);
            }
        }
        write.commit()?;

        Ok::<(), Error>(())
    })
    .await?
}
