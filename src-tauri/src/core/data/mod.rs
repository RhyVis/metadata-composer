use std::{
    collections::HashSet,
    fs,
    path::Path,
    sync::{Arc, OnceLock, RwLock},
};

use anyhow::{Result, anyhow};
use const_format::formatc;
use log::{debug, error, info, warn};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use tauri::async_runtime;
use tokio::fs as tfs;

use crate::{
    cmd::append::DeployArg,
    core::{
        data::metadata::{Metadata, MetadataOption},
        util::config::{InternalConfig, get_config, get_config_copy},
    },
};

pub mod metadata;

pub fn init_data() -> Result<()> {
    init_library()?;
    Ok(())
}

const LIB_FILE_STEM: &str = "lib";
const LIB_FILE_EXT: &str = "bin";
const LIB_FILE_NAME: &str = formatc!("{LIB_FILE_STEM}.{LIB_FILE_EXT}");
const LIB_FILE_EXPORT_EXT: &str = "json";
const LIB_FILE_EXPORT_NAME: &str = formatc!("{LIB_FILE_STEM}.{LIB_FILE_EXPORT_EXT}");

const DIR_BACKUP: &str = "backup";

const TABLE_METADATA: TableDefinition<&str, Vec<u8>> = TableDefinition::new("metadata");

static DB: OnceLock<Arc<Database>> = OnceLock::new();

static COLLECTION_TEMP: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();

fn init_library() -> Result<()> {
    let config = get_config_copy()?;

    fn create_db(config: &InternalConfig) -> Result<Database> {
        let db_path = config.root_data().join(LIB_FILE_NAME);
        Ok(Database::create(db_path).map_err(|e| anyhow!("Error creating the database: {}", e))?)
    }

    fn configure_db(db: Database) -> Result<Database> {
        let write = db.begin_write()?;
        write.open_table(TABLE_METADATA)?.get("TEST")?;
        write.commit()?;
        Ok(db)
    }

    fn backup_db(config: &InternalConfig) -> Result<()> {
        let backup_dir = config.root_data().join(DIR_BACKUP);
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }

        let db_path = config.root_data().join(LIB_FILE_NAME);
        if !db_path.exists() {
            return Ok(());
        }

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("{}_{}.{}", LIB_FILE_STEM, timestamp, LIB_FILE_EXT);
        let backup_path = backup_dir.join(&backup_filename);

        fs::copy(&db_path, &backup_path)?;
        info!("Database backup created at: {}", backup_path.display());

        let pattern = format!("{}_{}.{}", LIB_FILE_STEM, "*", LIB_FILE_EXT);
        let entries = fs::read_dir(&backup_dir)?
            .filter_map(Result::ok)
            .filter(|entry| {
                if let Some(name) = entry.file_name().to_str() {
                    return glob::Pattern::new(&pattern).unwrap().matches(name);
                }
                false
            })
            .collect::<Vec<_>>();

        if entries.len() > 5 {
            let mut entries_with_time = entries
                .iter()
                .filter_map(|entry| {
                    entry
                        .metadata()
                        .ok()
                        .and_then(|meta| meta.modified().ok().map(|time| (entry.path(), time)))
                })
                .collect::<Vec<_>>();

            entries_with_time.sort_by_key(|(_, time)| *time);

            for (path, _) in entries_with_time.iter().take(entries_with_time.len() - 5) {
                if let Err(e) = fs::remove_file(path) {
                    warn!("Failed to remove old backup file {}: {}", path.display(), e);
                } else {
                    info!("Removed old backup file: {}", path.display());
                }
            }
        }

        Ok(())
    }

    backup_db(&config)?;

    DB.set(Arc::new(configure_db(create_db(&config)?)?))
        .expect("Library database already initialized");
    COLLECTION_TEMP
        .set(RwLock::new(HashSet::new()))
        .map_err(|_| anyhow!("Failed to initialize collection temporary storage"))?;

    sync_collection_all()?;

    Ok(())
}

pub async fn export_library() -> Result<()> {
    let path = get_config()?.root_data().join(LIB_FILE_EXPORT_NAME);
    let all = metadata_get_all().await?;
    let json = serde_json::to_string(&all)
        .map_err(|e| anyhow!("Failed to serialize metadata to JSON: {}", e))?;

    tfs::write(&path, json).await?;

    info!("Library exported to {}", path.display());

    Ok(())
}

pub async fn import_library() -> Result<()> {
    let path = get_config()?.root_data().join(LIB_FILE_EXPORT_NAME);
    if !path.exists() {
        return Err(anyhow!("Library export file not found: {}", path.display()));
    }

    let json = tfs::read_to_string(&path)
        .await
        .map_err(|e| anyhow!("Failed to read library export file: {}", e))?;

    let entries: Vec<Metadata> =
        serde_json::from_str(&json).map_err(|e| anyhow!("Failed to deserialize JSON: {}", e))?;

    if entries.is_empty() {
        warn!(
            "No entries found in the library export file: {}",
            path.display()
        );
        return Ok(());
    }

    let db = get_db_copy()?;
    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            for entry in &entries {
                let raw =
                    bson::to_vec(entry).map_err(|e| anyhow!("Failed to serialize entry: {}", e))?;
                table.insert(&*entry.id.to_string(), raw)?;
            }
        }
        write.commit()?;
        Ok::<(), anyhow::Error>(())
    })
    .await??;

    info!("Library imported from {}", path.display());

    Ok(())
}

fn get_db() -> Result<&'static Database> {
    DB.get()
        .ok_or_else(|| anyhow!("Library database not initialized"))
        .map(|db| db.as_ref())
}

fn get_db_copy() -> Result<Arc<Database>> {
    DB.get()
        .cloned()
        .ok_or_else(|| anyhow!("Library database not initialized"))
}

async fn metadata_get_internal(key: &str) -> Result<Option<Metadata>> {
    let db = get_db_copy()?;
    let key = key.to_string();

    async_runtime::spawn_blocking(move || {
        let read = db.begin_read()?;
        let table = read.open_table(TABLE_METADATA)?;

        match table.get(&*key)? {
            Some(value) => match bson::from_slice(value.value().as_slice()) {
                Ok(value) => Ok(Some(value)),
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

async fn metadata_set_internal(key: &str, value: Metadata) -> Result<()> {
    let db = get_db_copy()?;
    let key = key.to_string();

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

async fn metadata_create_internal(opt: MetadataOption) -> Result<String> {
    let new = Metadata::create(opt).await?;
    let new_id = new.id.clone().to_string();
    sync_collection(&new)?;
    let db = get_db()?;
    let write = db.begin_write()?;
    {
        let mut table = write.open_table(TABLE_METADATA)?;
        let value = bson::to_vec(&new)
            .map_err(|e| anyhow!("Failed to serialize value for id '{}': {}", new_id, e))?;
        table.insert(new_id.as_str(), value)?;
    }
    write.commit()?;
    info!("Created new library entry with id '{}'", new_id);
    Ok(new_id)
}

async fn metadata_patch_internal(key: String, opt: MetadataOption) -> Result<()> {
    let db = get_db()?;
    if let Some(mut exist) = metadata_get_internal(key.as_str()).await? {
        let exist_archive_info = exist.archive_info.clone();
        exist.patch(opt).await?;

        if exist_archive_info.eq(&exist.archive_info) {
            debug!("No changes detected in archive info for key '{}'", key);
        } else {
            debug!(
                "Archive info changed for key '{}': {:?}",
                key, exist.archive_info
            );
            exist.archive_info.update_size().await?;
        }

        sync_collection(&exist)?;
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            let value = bson::to_vec(&exist)
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

async fn metadata_delete_internal(key: &str) -> Result<()> {
    let db = get_db_copy()?;
    let key = key.to_string();

    async_runtime::spawn_blocking(move || {
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_METADATA)?;
            if table.remove(&*key)?.is_none() {
                warn!("Key '{}' not found in library", key);
            } else {
                info!("Deleted library entry with id '{}'", key);
            }
        }
        write.commit()?;
        sync_collection_all()?;

        Ok::<(), anyhow::Error>(())
    })
    .await?
}

fn metadata_collection_list_internal() -> Result<Vec<String>> {
    Ok(COLLECTION_TEMP
        .get()
        .ok_or_else(|| anyhow!("Collection temp set not initialized"))?
        .read()
        .map_err(|e| anyhow!("Failed to read collection temp set: {}", e))?
        .iter()
        .cloned()
        .collect::<Vec<_>>())
}

fn sync_collection(data: &Metadata) -> Result<()> {
    let mut collections = COLLECTION_TEMP
        .get()
        .ok_or_else(|| anyhow!("Collection temp set not initialized"))?
        .write()
        .map_err(|e| anyhow!("Failed to write to collection temp set: {}", e))?;
    if let Some(collection_name) = data.collection.clone() {
        collections.insert(collection_name);
    }
    Ok(())
}

fn sync_collection_all() -> Result<()> {
    let db = get_db()?;
    let read = db.begin_read()?;
    let table = read.open_table(TABLE_METADATA)?;

    for (k, v) in table.iter()?.flatten() {
        if let Ok(data) = bson::from_slice::<Metadata>(v.value().as_slice()) {
            sync_collection(&data)?;
        } else {
            warn!("Failed to deserialize metadata for key '{}'", k.value());
        }
    }
    Ok(())
}

pub async fn metadata_update(opt: MetadataOption) -> Result<Option<String>> {
    if let Some(id) = &opt.id {
        metadata_patch_internal((*id).to_string(), opt)
            .await
            .map(|_| None)
    } else {
        metadata_create_internal(opt).await.map(Some)
    }
}

pub async fn metadata_get_all() -> Result<Vec<Metadata>> {
    let db = get_db_copy()?;
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

pub async fn metadata_get(key: &str) -> Result<Option<Metadata>> {
    metadata_get_internal(key).await
}

pub async fn metadata_delete(key: &str) -> Result<()> {
    metadata_delete_internal(key).await
}

pub fn metadata_collection_list() -> Result<Vec<String>> {
    metadata_collection_list_internal()
}

pub async fn metadata_deploy(key: &str, arg: DeployArg) -> Result<()> {
    let data = metadata_get_internal(key).await?;
    if let Some(mut data) = data {
        let deploy_dir = get_config()?.root_deploy().cloned();
        if arg.use_config_dir {
            // Deploy to configured dir
            let mut deploy_dir = deploy_dir.expect("Why?");
            info!("Deploy to configured root: {}", deploy_dir.display());
            deploy_dir.push(&data.title);
            fs::create_dir_all(&deploy_dir)?;
            let id = data.id.to_string();
            if data.deploy(deploy_dir).await? {
                info!("Successfully deployed metadata with id '{id}'");
                metadata_set_internal(&id, data).await?;
                Ok(())
            } else {
                Err(anyhow!("Failed to deploy metadata with id '{id}'"))
            }
        } else if arg.target_dir.is_some() {
            // Deploy to custom dir
            let target_dir = arg.target_dir.expect("Why?");
            let target_dir = Path::new(target_dir.as_str());
            info!("Deploy to custom dir: {}", target_dir.display());
            if !target_dir.exists() {
                fs::create_dir_all(target_dir)?;
            }
            let id = data.id.to_string();
            if data.deploy(target_dir).await? {
                info!("Successfully deployed metadata with id '{id}'");
                metadata_set_internal(&id, data).await?;

                Ok(())
            } else {
                Err(anyhow!("Failed to deploy metadata with id '{id}'"))
            }
        } else {
            error!("Provided deploy argument not valid: {}, {:?}", key, arg);
            Err(anyhow!("Failed to deploy metadata with key '{}'", key))
        }
    } else {
        Err(anyhow!("Key '{}' not found in library", key))
    }
}

pub async fn metadata_deploy_off(key: &str) -> Result<()> {
    let data = metadata_get_internal(key).await?;
    if let Some(mut data) = data {
        if data.deploy_off().await? {
            info!("Successfully deployed metadata with key '{key}'");
            metadata_set_internal(key, data).await?;
            Ok(())
        } else {
            Err(anyhow!("Failed to deploy metadata with key '{key}'"))
        }
    } else {
        Err(anyhow!("Key '{}' not found in library", key))
    }
}

pub async fn clear_unused_images() -> Result<u32> {
    let all_used_images = metadata_get_all()
        .await?
        .iter()
        .map(|data| data.image.clone())
        .flatten()
        .collect::<HashSet<_>>();
    let dir_image = get_config()?.dir_image();
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
                    tfs::remove_file(&path).await?;
                    debug!("Removed unused image: {}", path.display());
                    removed_count += 1;
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
