use anyhow::{Result, anyhow};
use log::warn;
use redb::ReadableTable;
use tauri::State;

use crate::core::data::{library::TABLE_METADATA, metadata::Metadata, state::DataState};

pub(super) fn collection_cache_sync(metadata: &Metadata, data: State<'_, DataState>) -> Result<()> {
    if let Some(name) = &metadata.collection {
        data.collection_cache_edit(|c| {
            c.insert(name.clone());
        })?;
    }
    Ok(())
}

pub(super) fn collection_cache_sync_all(data: State<'_, DataState>) -> Result<()> {
    let db = data.database();
    let mut collections = data
        .collection_cache()
        .lock()
        .map_err(|e| anyhow!("Failed to lock collection temp set: {}", e))?;

    let read = db.begin_read()?;
    let table = read.open_table(TABLE_METADATA)?;

    for (k, v) in table.iter()?.flatten() {
        if let Ok(data) = bson::from_slice::<Metadata>(v.value().as_slice()) {
            if let Some(collection) = data.collection {
                collections.insert(collection);
            }
        } else {
            warn!("Failed to deserialize metadata for key '{}'", k.value());
        }
    }

    Ok(())
}

pub(super) fn collection_cache_remove(
    metadata: &Metadata,
    data: State<'_, DataState>,
) -> Result<()> {
    if let Some(name) = &metadata.collection {
        data.collection_cache_edit(|c| {
            c.remove(name);
        })?;
    }
    Ok(())
}
