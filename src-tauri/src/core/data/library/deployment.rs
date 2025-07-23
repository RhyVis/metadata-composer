use anyhow::{Result, anyhow};
use log::warn;
use redb::ReadableTable;
use tauri::State;

use crate::core::data::{
    metadata::{DeployInfo, Metadata},
    state::DataState,
};

pub(super) fn deployment_cache_sync(metadata: &Metadata, data: State<'_, DataState>) -> Result<()> {
    if DeployInfo::None == metadata.deploy_info {
        return Ok(());
    }
    data.deployment_cache_edit(|c| {
        c.insert(metadata.id.to_string());
    })
}

pub(super) fn deployment_cache_sync_all(data: State<'_, DataState>) -> Result<()> {
    let db = data.database();
    let mut deployments = data
        .deployment_cache()
        .lock()
        .map_err(|e| anyhow!("Failed to lock deployment temp set: {}", e))?;

    let read = db.begin_read()?;
    let table = read.open_table(crate::core::data::library::TABLE_METADATA)?;

    for (k, v) in table.iter()?.flatten() {
        if let Ok(data) = bson::from_slice::<Metadata>(v.value().as_slice()) {
            if data.deploy_info != DeployInfo::None {
                deployments.insert(data.id.to_string());
            }
        } else {
            warn!("Failed to deserialize metadata for key '{}'", k.value());
        }
    }

    Ok(())
}

pub(super) fn deployment_cache_remove(
    metadata: &Metadata,
    data: State<'_, DataState>,
) -> Result<()> {
    if DeployInfo::None == metadata.deploy_info {
        return Ok(());
    }
    data.deployment_cache_edit(|c| {
        c.remove(&metadata.id.to_string());
    })
}
