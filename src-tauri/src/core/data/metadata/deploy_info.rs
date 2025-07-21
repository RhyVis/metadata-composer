use crate::core::Whether;
use crate::core::Whether::{That, This};
use log::warn;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum DeployInfo {
    #[default]
    None,
    File {
        path: PathBuf,
    },
    Directory {
        path: PathBuf,
    },
}

impl DeployInfo {
    pub(super) fn new_file(path: PathBuf) -> Self {
        DeployInfo::File { path }
    }

    pub(super) fn new_dir(path: PathBuf) -> Self {
        DeployInfo::Directory { path }
    }

    pub(super) fn try_resolve(&self) -> Whether<PathBuf, DeployInfo> {
        match self {
            DeployInfo::None => {
                warn!("DeployInfo is unset, cannot resolve path.");
                That(DeployInfo::None)
            }
            DeployInfo::File { path } => {
                if path.exists() {
                    This(path.clone())
                } else {
                    warn!("The specified file path does not exist: {}", path.display());
                    That(DeployInfo::None)
                }
            }
            DeployInfo::Directory { path } => {
                if path.exists() {
                    This(path.clone())
                } else {
                    warn!(
                        "The specified directory path does not exist: {}",
                        path.display()
                    );
                    That(DeployInfo::None)
                }
            }
        }
    }
}
