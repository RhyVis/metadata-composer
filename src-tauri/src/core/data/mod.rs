use crate::core::data::metadata::{Metadata, MetadataOption};
use crate::core::util::config::get_config;
use anyhow::{Result, anyhow};
use const_format::formatc;
use log::{info, warn};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use std::collections::HashSet;
use std::sync::{OnceLock, RwLock};

pub mod collection;
pub mod metadata;

pub fn init_data() -> Result<()> {
    init_library()?;
    Ok(())
}

const LIB_FILE_STEM: &str = "lib";
const LIB_FILE_EXT: &str = "bin";
const LIB_FILE_NAME: &str = formatc!("{LIB_FILE_STEM}.{LIB_FILE_EXT}");

const TABLE_IMAGE: TableDefinition<&str, Vec<u8>> = TableDefinition::new("image");
const TABLE_METADATA: TableDefinition<&str, Vec<u8>> = TableDefinition::new("metadata");

static DB: OnceLock<Database> = OnceLock::new();

static COLLECTION_TEMP: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();

fn init_library() -> Result<()> {
    fn create_db() -> Result<Database> {
        let config = get_config()?;
        let db_path = config.root().join(LIB_FILE_NAME);
        Ok(Database::create(db_path)?)
    }

    fn configure_db(db: Database) -> Result<Database> {
        let write = db.begin_write()?;
        write.open_table(TABLE_IMAGE)?.get("TEST")?;
        write.open_table(TABLE_METADATA)?.get("TEST")?;
        write.commit()?;
        Ok(db)
    }

    DB.set(configure_db(create_db()?)?)
        .expect("Library database already initialized");
    COLLECTION_TEMP
        .set(RwLock::new(HashSet::new()))
        .map_err(|_| anyhow!("Failed to initialize collection temporary storage"))?;

    sync_collection_all()?;

    Ok(())
}

fn get_db() -> Result<&'static Database> {
    DB.get()
        .ok_or_else(|| anyhow!("Library database not initialized"))
}

fn metadata_get_internal(key: &str) -> Result<Option<Metadata>> {
    let db = get_db()?;
    let read = db.begin_read()?;
    let table = read.open_table(TABLE_METADATA)?;

    match table.get(key)? {
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
        }
    }
}

fn metadata_create_internal(opt: MetadataOption) -> Result<String> {
    let new = Metadata::create(opt)?;
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

fn metadata_patch_internal(key: String, opt: MetadataOption) -> Result<()> {
    let db = get_db()?;
    if let Some(mut exist) = metadata_get_internal(key.as_str())? {
        exist.patch(opt);
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

fn metadata_delete_internal(key: &str) -> Result<()> {
    let db = get_db()?;
    let write = db.begin_write()?;
    {
        let mut table = write.open_table(TABLE_METADATA)?;
        if table.remove(key)?.is_none() {
            warn!("Key '{}' not found in library", key);
        } else {
            info!("Deleted library entry with id '{}'", key);
        }
    }
    write.commit()?;
    Ok(())
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

    for entry in table.iter()? {
        if let Ok((key, value)) = entry {
            if let Ok(data) = bson::from_slice::<Metadata>(value.value().as_slice()) {
                sync_collection(&data)?;
            } else {
                warn!("Failed to deserialize metadata for key '{}'", key.value());
            }
        }
    }
    Ok(())
}

pub fn metadata_update(opt: MetadataOption) -> Result<Option<String>> {
    if let Some(id) = &opt.id {
        metadata_patch_internal((*id).to_string(), opt).map(|_| None)
    } else {
        metadata_create_internal(opt).map(Some)
    }
}

pub fn metadata_get_all() -> Result<Vec<Metadata>> {
    let db = get_db()?;
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
            bson::from_slice(v.value().as_slice())
                .map_err(|e| anyhow!("Failed to deserialize value for key '{}': {}", k.value(), e))
        })
        .collect::<Result<Vec<Metadata>>>()
}

pub fn metadata_get(key: &str) -> Result<Option<Metadata>> {
    metadata_get_internal(key)
}

pub fn metadata_delete(key: &str) -> Result<()> {
    metadata_delete_internal(key)
}

pub fn metadata_collection_list() -> Result<Vec<String>> {
    metadata_collection_list_internal()
}
