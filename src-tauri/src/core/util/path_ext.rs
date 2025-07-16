use log::warn;
use std::path::Path;
use walkdir::WalkDir;

pub trait PathExt {
    fn calculate_size(&self) -> u64;
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
                }
            }
        } else if self.is_dir() {
            let walker = WalkDir::new(&self);
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
                        }
                    }
                }
            }

            total_size
        } else {
            warn!("Path is neither a file nor a directory: {}", self.display());
            0
        }
    }
}
