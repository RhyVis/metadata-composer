mod collection;
mod delegate;
mod deployment;
mod util;

use std::{fs, path::Path};

use anyhow::{Result, anyhow};
use const_format::formatc;
use log::{info, warn};
use redb::{Database, ReadableTable, TableDefinition};
use tauri::{AppHandle, Manager, async_runtime};
use tokio::fs as tfs;

pub use self::{delegate::*, deployment::*, util::clear_unused_images};
use crate::core::{
    AppStateExt,
    data::{library::collection::collection_cache_sync_all, metadata::Metadata, state::DataState},
};

const LIB_FILE_STEM: &str = "lib";
const LIB_FILE_EXT: &str = "bin";
const LIB_FILE_NAME: &str = formatc!("{LIB_FILE_STEM}.{LIB_FILE_EXT}");
const LIB_FILE_EXPORT_EXT: &str = "json";
const LIB_FILE_EXPORT_NAME: &str = formatc!("{LIB_FILE_STEM}.{LIB_FILE_EXPORT_EXT}");

const DIR_BACKUP: &str = "backup";

const TABLE_METADATA: TableDefinition<&str, Vec<u8>> = TableDefinition::new("metadata");

pub(super) fn init_library(app: &AppHandle) -> Result<()> {
    let config = app.state_config().get();
    let path_data = config.path_data();

    fn create_db(path_data: &Path) -> Result<Database> {
        let db_path = path_data.join(LIB_FILE_NAME);
        Ok(Database::create(db_path).map_err(|e| anyhow!("Error creating the database: {}", e))?)
    }

    fn configure_db(db: Database) -> Result<Database> {
        let write = db.begin_write()?;
        write.open_table(TABLE_METADATA)?.get("TEST")?;
        write.commit()?;
        Ok(db)
    }

    fn backup_db(path_data: &Path) -> Result<()> {
        let path_data = path_data.to_path_buf();

        let backup_dir = path_data.join(DIR_BACKUP);
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }

        let db_path = path_data.join(LIB_FILE_NAME);
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

    backup_db(path_data)?;
    app.manage(DataState::new(configure_db(create_db(path_data)?)?));
    collection_cache_sync_all(app.state_data())?;

    Ok(())
}

pub async fn export_library(app: AppHandle) -> Result<()> {
    let path = app
        .state_config()
        .get()
        .path_data()
        .join(LIB_FILE_EXPORT_NAME);
    let all = metadata_get_all(app.state_data()).await?;
    let json = serde_json::to_string(&all)
        .map_err(|e| anyhow!("Failed to serialize metadata to JSON: {}", e))?;

    tfs::write(&path, json).await?;

    info!("Library exported to {}", path.display());

    Ok(())
}

pub async fn import_library(app: AppHandle) -> Result<()> {
    let path = app
        .state_config()
        .get()
        .path_data()
        .join(LIB_FILE_EXPORT_NAME);
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

    let db = app.state_data().database();
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
