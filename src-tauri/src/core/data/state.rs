use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use anyhow::{Result, anyhow};
use redb::Database;

pub struct DataState {
    database: Arc<Database>,
    collection_cache: Mutex<HashSet<String>>,
    deployment_cache: Mutex<HashSet<String>>,
}

impl DataState {
    pub(super) fn new(database: Database) -> Self {
        Self {
            database: Arc::new(database),
            collection_cache: Mutex::new(HashSet::new()),
            deployment_cache: Mutex::new(HashSet::new()),
        }
    }

    /// Directly returns a reference to the database.
    pub fn database_ref(&self) -> &Database {
        &self.database
    }

    /// Returns a clone of the database wrapped in an `Arc`.
    /// Use this across threads to avoid ownership issues.
    pub fn database(&self) -> Arc<Database> {
        Arc::clone(&self.database)
    }

    pub fn collection_cache(&self) -> &Mutex<HashSet<String>> {
        &self.collection_cache
    }

    pub fn collection_cache_edit(&self, f: impl FnOnce(&mut HashSet<String>)) -> Result<()> {
        let mut collections = self
            .collection_cache
            .lock()
            .map_err(|e| anyhow!("Failed to lock collection temp set: {}", e))?;
        f(&mut collections);
        Ok(())
    }

    pub fn deployment_cache(&self) -> &Mutex<HashSet<String>> {
        &self.deployment_cache
    }

    pub fn deployment_cache_edit(&self, f: impl FnOnce(&mut HashSet<String>)) -> Result<()> {
        let mut deployments = self
            .deployment_cache
            .lock()
            .map_err(|e| anyhow!("Failed to lock deployment temp set: {}", e))?;
        f(&mut deployments);
        Ok(())
    }
}
