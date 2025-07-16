use crate::core::data::metadata::{Metadata, MetadataOption};
use crate::core::util::config::get_config;
use anyhow::{Result, anyhow};
use const_format::formatc;
use log::{info, warn};
use redb::{Database, ReadableTable, TableDefinition};
use std::sync::OnceLock;

const LIBRARY_FILE_STEM: &str = "library";
const LIBRARY_FILE_EXT: &str = "bin";
const LIBRARY_FILE_NAME: &str = formatc!("{LIBRARY_FILE_STEM}.{LIBRARY_FILE_EXT}");

const TABLE_LIBRARY: TableDefinition<&str, Vec<u8>> = TableDefinition::new("library");

static DB: OnceLock<Database> = OnceLock::new();

pub(super) fn init_library() -> Result<()> {
    fn create_db() -> Result<Database> {
        let config = get_config()?;
        let db_path = config.root.join(LIBRARY_FILE_NAME);
        Ok(Database::create(db_path)?)
    }

    fn configure_db(db: Database) -> Result<Database> {
        let write = db.begin_write()?;
        write.open_table(TABLE_LIBRARY)?.get("TEST")?;
        write.commit()?;
        Ok(db)
    }

    DB.set(configure_db(create_db()?)?)
        .expect("Library database already initialized");
    Ok(())
}

fn get_db() -> Result<&'static Database> {
    DB.get()
        .ok_or_else(|| anyhow!("Library database not initialized"))
}

fn lib_get_internal(key: &str) -> Result<Option<Metadata>> {
    let db = get_db()?;
    let read = db.begin_read()?;
    let table = read.open_table(TABLE_LIBRARY)?;

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

fn lib_create_internal(opt: MetadataOption) -> Result<String> {
    let new = Metadata::create(opt);
    let new_id = new.id.clone();
    let db = get_db()?;
    let write = db.begin_write()?;
    {
        let mut table = write.open_table(TABLE_LIBRARY)?;
        let value = bson::to_vec(&new)
            .map_err(|e| anyhow!("Failed to serialize value for id '{}': {}", new_id, e))?;
        table.insert(&*new_id, value)?;
    }
    info!("Created new library entry with id '{}'", new_id);
    Ok(new_id)
}

fn lib_patch_internal(key: String, opt: MetadataOption) -> Result<()> {
    let db = get_db()?;
    if let Some(mut exist) = lib_get_internal(key.as_str())? {
        exist.patch(opt);
        let write = db.begin_write()?;
        {
            let mut table = write.open_table(TABLE_LIBRARY)?;
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

pub fn lib_update(opt: MetadataOption) -> Result<Option<String>> {
    if let Some(id) = &opt.id {
        lib_patch_internal((*id).clone(), opt).map(|_| None)
    } else {
        lib_create_internal(opt).map(Some)
    }
}

pub fn lib_get_all() -> Result<Vec<Metadata>> {
    let db = get_db()?;
    let read = db.begin_read()?;
    let table = read.open_table(TABLE_LIBRARY)?;

    table
        .iter()?
        .filter_map(Result::ok)
        .map(|(k, v)| {
            bson::from_slice(v.value().as_slice())
                .map_err(|e| anyhow!("Failed to deserialize value for key '{}': {}", k.value(), e))
        })
        .collect::<Result<Vec<Metadata>>>()
}

pub fn lib_get(key: &str) -> Result<Option<Metadata>> {
    lib_get_internal(key)
}
