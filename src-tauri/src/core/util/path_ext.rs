use std::{fs, path::Path};

use log::warn;
use tokio::fs as tfs;
use tokio_stream::StreamExt;

#[allow(dead_code)]
pub trait PathExt {
    /// It does not check if the path exists, it just returns 0 if the path does
    /// not exist, or encountered an error.
    fn calculate_size(&self) -> u64;

    fn calculate_size_async(&self) -> impl Future<Output = u64> + Send;

    /// Checks if the directory is empty.
    /// If not a directory, it returns false.
    fn is_dir_empty(&self) -> bool;

    fn is_dir_empty_async(&self) -> impl Future<Output = bool> + Send;

    /// Clears a path if it's a dir
    fn clear_dir(&self) -> std::io::Result<()>;

    fn clear_dir_async(&self) -> impl Future<Output = std::io::Result<()>> + Send;
}

impl PathExt for Path {
    fn calculate_size(&self) -> u64 {
        if !self.exists() {
            warn!(
                "Tried to calculate size of non-existent path: {}",
                self.display()
            );
            return 0;
        }

        if self.is_file() {
            match self.metadata() {
                Ok(metadata) => metadata.len(),
                Err(e) => {
                    warn!("Failed to get metadata for file {}: {}", self.display(), e);
                    0
                },
            }
        } else if self.is_dir() {
            let walker = walkdir::WalkDir::new(self);
            let mut total_size = 0;

            for entry in walker.into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    match entry.metadata() {
                        Ok(metadata) => total_size += metadata.len(),
                        Err(e) => {
                            warn!(
                                "Failed to get metadata for file {}: {}",
                                entry.path().display(),
                                e
                            );
                        },
                    }
                }
            }

            total_size
        } else {
            warn!("Path is neither a file nor a directory: {}", self.display());
            0
        }
    }

    async fn calculate_size_async(&self) -> u64 {
        if !tfs::try_exists(self).await.unwrap_or(false) {
            warn!(
                "Tried to calculate size of non-existent path: {}",
                self.display()
            );
            return 0;
        }

        if self.is_file() {
            match tfs::metadata(self).await {
                Ok(metadata) => metadata.len(),
                Err(e) => {
                    warn!("Failed to get metadata for file {}: {}", self.display(), e);
                    0
                },
            }
        } else if self.is_dir() {
            let mut total_size = 0;

            while let Some(entry) = async_walkdir::WalkDir::new(self)
                .filter_map(Result::ok)
                .next()
                .await
            {
                if entry.file_type().await.map_or(false, |ft| ft.is_file()) {
                    match entry.metadata().await {
                        Ok(metadata) => total_size += metadata.len(),
                        Err(e) => {
                            warn!(
                                "Failed to get metadata for file {}: {}",
                                entry.path().display(),
                                e
                            );
                        },
                    }
                }
            }

            total_size
        } else {
            warn!("Path is neither a file nor a directory: {}", self.display());
            0
        }
    }

    fn is_dir_empty(&self) -> bool {
        if !self.is_dir() {
            warn!("Path is not a directory: {}", self.display());
            return false;
        }

        match fs::read_dir(self) {
            Ok(mut entries) => entries.next().is_none(),
            Err(e) => {
                warn!("Failed to read directory {}: {}", self.display(), e);
                false
            },
        }
    }

    async fn is_dir_empty_async(&self) -> bool {
        if !self.is_dir() {
            warn!("Path is not a directory: {}", self.display());
            return false;
        }

        match tfs::read_dir(self).await {
            Ok(mut entries) => entries.next_entry().await.map_or(false, |e| e.is_none()),
            Err(e) => {
                warn!("Failed to read directory {}: {}", self.display(), e);
                false
            },
        }
    }

    fn clear_dir(&self) -> std::io::Result<()> {
        if !self.is_dir() {
            warn!("Path is not a directory: {}", self.display());
            return Ok(());
        }

        let read_dir = fs::read_dir(self)?;
        for entry in read_dir {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                fs::remove_file(entry.path())?;
            } else if entry.file_type()?.is_dir() {
                fs::remove_dir_all(entry.path())?;
            }
        }

        Ok(())
    }

    async fn clear_dir_async(&self) -> std::io::Result<()> {
        let path = self.to_owned();
        if !self.is_dir() {
            warn!("Path is not a directory: {}", self.display());
            return Ok(());
        }

        let mut read_dir = tfs::read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            if entry.file_type().await?.is_file() {
                tfs::remove_file(entry.path()).await?;
            } else if entry.file_type().await?.is_dir() {
                tfs::remove_dir_all(entry.path()).await?;
            }
        }

        Ok(())
    }
}
