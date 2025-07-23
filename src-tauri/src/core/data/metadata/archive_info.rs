use std::path::{Path, PathBuf};

use log::{info, warn};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::core::{
    Whether,
    Whether::{That, This},
    util::{config::get_config_copy, path_ext::PathExt},
};

/// Represents archive information for a data item, such as size and path
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum ArchiveInfo {
    #[default]
    None,
    ArchiveFile {
        #[ts(type = "number")]
        size: u64,
        path: String,
        password: Option<String>,
    },
    CommonFile {
        #[ts(type = "number")]
        size: u64,
        path: String,
    },
    Directory {
        #[ts(type = "number")]
        size: u64,
        path: String,
    },
}

impl ArchiveInfo {
    pub(super) fn try_resolve(&self) -> anyhow::Result<Whether<PathBuf, Self>> {
        match self {
            ArchiveInfo::None => {
                warn!("Trying to resolve None archive info, returning self");
                Ok(That(ArchiveInfo::None))
            },
            ArchiveInfo::ArchiveFile { path, .. } => {
                let path_seg = Path::new(path);
                let mut path_base = get_config_copy()?.dir_archive();
                path_base.push(path_seg);
                if path_base.exists() {
                    Ok(This(path_base))
                } else {
                    warn!(
                        "The specified archive file does not exist: {}",
                        path_base.display()
                    );
                    Ok(That(ArchiveInfo::None))
                }
            },
            ArchiveInfo::CommonFile { path, .. } => {
                let path = Path::new(path);
                if path.exists() {
                    Ok(This(path.to_owned()))
                } else {
                    warn!(
                        "The specified common file does not exist: {}",
                        path.display()
                    );
                    Ok(That(ArchiveInfo::None))
                }
            },
            ArchiveInfo::Directory { path, .. } => {
                let path = Path::new(path);
                if path.exists() && path.is_dir() {
                    Ok(This(path.to_owned()))
                } else {
                    warn!(
                        "The specified directory does not exist or is not a directory: {}",
                        path.display()
                    );
                    Ok(That(ArchiveInfo::None))
                }
            },
        }
    }

    pub async fn update_size(&mut self) -> anyhow::Result<()> {
        let path = match self.try_resolve()? {
            This(path) => path,
            That(_) => {
                warn!("ArchiveInfo is not resolved, cannot update size.");
                return Ok(());
            },
        };

        let size = path.calculate_size_async().await;
        match self {
            ArchiveInfo::ArchiveFile { size: s, .. } => *s = size,
            ArchiveInfo::CommonFile { size: s, .. } => *s = size,
            ArchiveInfo::Directory { size: s, .. } => *s = size,
            ArchiveInfo::None => unreachable!(),
        }
        info!("Updated archive info size to: {}", size);
        Ok(())
    }
}
