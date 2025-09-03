use std::path::Path;

use anyhow::{Result, anyhow};
use log::{debug, error, info, warn};
use redb::{ReadableTable, ReadableTableMetadata};
use tauri::{AppHandle, State, async_runtime};
use tokio::fs as tfs;

use crate::{
    cmd::append::DeployArg,
    core::{
        AppStateExt,
        data::{
            library::{
                TABLE_METADATA, clear_unused_deploy_dirs,
                collection::{collection_cache_remove, collection_cache_sync},
                deployment::{deployment_cache_remove, deployment_cache_sync},
            },
            metadata::{Metadata, MetadataOption},
            state::DataState,
        },
        get_handle, get_handle_ref,
    },
};

async fn metadata_get_internal(
    key: String,
    data: State<'_, DataState>,
) -> Result<Option<Metadata>> {
    let db = data.database();

    async_runtime::spawn_blocking(move || {
        let read = db.begin_read()?;
        let table = read.open_table(TABLE_METADATA)?;

        match table.get(&*key)? {
            Some(value) => match bson::from_slice(value.value().as_slice()) {
                Ok(metadata) => Ok(Some(metadata)),
                Err(e) => Err(anyhow!(
                    "Failed to deserialize value for key '{}': {}",
                    key,
                    e
                )),
            },
            None => {
                warn!("Key '{}' not found in library", key);
                Ok(None)
            },
        }
    })
    .await?
}

async fn metadata_set_internal(
    key: String,
    value: Metadata,
    data: State<'_, DataState>,
) -> Result<()> {
    let db = data.database();

    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            let raw = bson::to_vec(&value)?;
            table.insert(&*key, raw)?;
        }
        write.commit()?;
        Ok(())
    })
    .await?
}

async fn metadata_create_internal(
    opt: MetadataOption,
    data: State<'_, DataState>,
) -> Result<String> {
    let new = Metadata::create(opt).await?;
    let new_id = new.id.to_string();
    let new_id_clone = new_id.clone();

    collection_cache_sync(&new, data.clone())?;
    deployment_cache_sync(&new, data.clone())?;

    let db = data.database();
    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            let value = bson::to_vec(&new)
                .map_err(|e| anyhow!("Failed to serialize value for id '{}': {}", new_id, e))?;
            table.insert(new_id.as_str(), value)?;
        }
        write.commit()?;
        Ok::<(), anyhow::Error>(())
    })
    .await??;

    info!("Created new library entry with id '{}'", new_id_clone);
    Ok(new_id_clone)
}

async fn metadata_patch_internal(
    key: String,
    opt: MetadataOption,
    data: State<'_, DataState>,
) -> Result<()> {
    let db = data.database();
    if let Some(mut metadata) = metadata_get_internal(key.clone(), data.clone()).await? {
        let exist_archive_info = metadata.archive_info.clone();
        metadata.patch(opt).await?;

        if exist_archive_info.eq(&metadata.archive_info) {
            debug!("No changes detected in archive info for key '{}'", key);
        } else {
            debug!(
                "Archive info changed for key '{}': {:?}",
                key, metadata.archive_info
            );
            metadata.archive_info.update_size().await?;
        }

        collection_cache_sync(&metadata, data.clone())?;
        deployment_cache_sync(&metadata, data)?;

        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            let value = bson::to_vec(&metadata)
                .map_err(|e| anyhow!("Failed to serialize value for key '{}': {}", key, e))?;
            table.insert(&*key, value)?;
        }
        write.commit()?;
        info!("Patched library entry with id '{}'", key);
        Ok(())
    } else {
        Err(anyhow!("Key '{}' not found in library", key))
    }
}

async fn metadata_delete_internal(key: String, data: State<'_, DataState>) -> Result<()> {
    let db = data.database();

    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        let removed = {
            let mut table = write.open_table(TABLE_METADATA)?;
            if let Some(removed) = table.remove(&*key)? {
                let removed = bson::from_slice::<Metadata>(removed.value().as_slice())?;
                info!("Deleted library entry with id '{}'", key);
                Some(removed)
            } else {
                warn!("Key '{}' not found in library", key);
                None
            }
        };
        write.commit()?;

        if let Some(removed) = removed {
            let data = get_handle_ref().state_data();
            let _ = collection_cache_remove(&removed, data.clone())
                .inspect_err(|e| error!("Failed to remove collection from cache: {}", e));
            let _ = deployment_cache_remove(&removed, data)
                .inspect_err(|e| error!("Failed to remove deployment from cache: {}", e));
        }

        Ok::<(), anyhow::Error>(())
    })
    .await?
}

fn metadata_collection_list_internal(data: State<'_, DataState>) -> Result<Vec<String>> {
    Ok(data
        .collection_cache()
        .lock()
        .map_err(|e| anyhow!("Failed to read collection temp set: {}", e))?
        .iter()
        .cloned()
        .collect::<Vec<_>>())
}

pub async fn metadata_update(
    opt: MetadataOption,
    data: State<'_, DataState>,
) -> Result<Option<String>> {
    if let Some(id) = &opt.id {
        metadata_patch_internal((*id).to_string(), opt, data)
            .await
            .map(|_| None)
    } else {
        metadata_create_internal(opt, data).await.map(Some)
    }
}

pub async fn metadata_get_all(data: State<'_, DataState>) -> Result<Vec<Metadata>> {
    let db = data.database();
    async_runtime::spawn_blocking(move || {
        let read = db.begin_read()?;
        let table = read.open_table(TABLE_METADATA)?;

        info!(
            "Retrieving all metadata entries from library: {}",
            table.len()?
        );
        table
            .iter()?
            .filter_map(Result::ok)
            .map(|(k, v)| {
                bson::from_slice(v.value().as_slice()).map_err(|e| {
                    anyhow!("Failed to deserialize value for key '{}': {}", k.value(), e)
                })
            })
            .collect::<Result<Vec<Metadata>>>()
    })
    .await?
}

pub async fn metadata_get(key: String, data: State<'_, DataState>) -> Result<Option<Metadata>> {
    metadata_get_internal(key, data).await
}

pub async fn metadata_delete(key: String, data: State<'_, DataState>) -> Result<()> {
    metadata_delete_internal(key, data).await
}

pub fn metadata_collection_list(data: State<'_, DataState>) -> Result<Vec<String>> {
    metadata_collection_list_internal(data)
}

pub async fn metadata_deploy(key: String, arg: DeployArg, app: AppHandle) -> Result<()> {
    let data = app.state_data();
    let config = app.state_config();

    let existing_data = metadata_get_internal(key.clone(), data.clone()).await?;
    if let Some(metadata) = existing_data {
        async fn deploy_process(
            mut metadata: Metadata,
            deploy_path: impl AsRef<Path>,
            app: &AppHandle,
        ) -> Result<()> {
            let id = metadata.id.to_string();
            if metadata.deploy(deploy_path, app).await? {
                info!("Successfully deployed metadata with id '{id}'");

                let data = app.state_data();

                metadata_set_internal(id, metadata.clone(), data.clone()).await?;
                let _ = deployment_cache_sync(&metadata, data).inspect_err(|e| {
                    error!(
                        "Failed to sync deployment cache for '{}': {}",
                        metadata.id, e
                    )
                });
                Ok(())
            } else {
                Err(anyhow!("Failed to deploy metadata with id '{id}'"))
            }
        }

        let deploy_dir = config.get().path_deploy().map(|p| p.to_path_buf());
        if arg.use_config_dir {
            // Deploy to configured dir
            let mut deploy_path = deploy_dir.expect("Why?");
            deploy_path.push(&metadata.title);
            info!("Deploy to configured path: {}", deploy_path.display());

            if !tfs::try_exists(&deploy_path).await? {
                tfs::create_dir_all(&deploy_path).await?;
            }

            deploy_process(metadata, deploy_path, &app).await
        } else if arg.target_dir.is_some() {
            // Deploy to custom dir
            let deploy_path = arg.target_dir.expect("Why?");
            let deploy_path = Path::new(deploy_path.as_str());
            info!("Deploy to custom dir: {}", deploy_path.display());

            if !tfs::try_exists(&deploy_path).await? {
                tfs::create_dir_all(&deploy_path).await?;
            }

            deploy_process(metadata, deploy_path, &app).await
        } else {
            error!("Provided deploy argument not valid: {}, {:?}", key, arg);
            Err(anyhow!("Failed to deploy metadata with key '{}'", key))
        }
    } else {
        Err(anyhow!("Key '{}' not found in library", key))
    }
}

pub async fn metadata_deploy_off(key: String, data: State<'_, DataState>) -> Result<()> {
    let existing_data = metadata_get_internal(key.clone(), data.clone()).await?;
    if let Some(mut metadata) = existing_data {
        if metadata.deploy_off().await? {
            metadata_set_internal(key.clone(), metadata.clone(), data.clone()).await?;
            let _ = deployment_cache_remove(&metadata, data).inspect_err(|e| {
                error!(
                    "Failed to remove deployment cache for '{}': {}",
                    metadata.id, e
                )
            });
            let _ = clear_unused_deploy_dirs(get_handle())
                .await
                .inspect_err(|e| error!("Failed to clear unused deploy dirs: {}", e));
            info!("Successfully deployed metadata with key '{key}'");
            Ok(())
        } else {
            Err(anyhow!("Failed to deploy metadata with key '{key}'"))
        }
    } else {
        Err(anyhow!("Key '{}' not found in library", key))
    }
}
