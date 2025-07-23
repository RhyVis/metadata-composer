use std::{ffi::OsStr, fs, path::Path};

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use fs_extra::{dir, dir::CopyOptions};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use tauri::async_runtime;
use tokio::fs as tfs;
use ts_rs::TS;
use uuid::Uuid;

use crate::core::{
    Whether::{That, This},
    util::{
        compress::{compress, decompress},
        config::get_config_copy,
        path_ext::PathExt,
    },
};

mod archive_info;
mod content_info;
mod deploy_info;

pub use archive_info::*;
pub use content_info::*;
pub use deploy_info::*;

/// Basic metadata structure for data item
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct Metadata {
    /// Unique identifier for the data item,
    /// by default a [Uuid]
    pub id: Uuid,

    /// The title of the data item
    #[serde(default)]
    pub title: String,
    /// The other names of the data item
    #[serde(default)]
    pub alias: Vec<String>,
    /// Tags associated with the data item
    #[serde(default)]
    pub tags: Vec<String>,
    /// Collection names, if any
    #[serde(default)]
    pub collection: Option<String>,
    /// Description of the data item
    #[serde(default)]
    pub description: Option<String>,
    /// Image hash, if any
    #[serde(default)]
    pub image: Option<String>,

    /// The content type of the data item
    #[serde(default)]
    pub content_info: ContentInfo,
    /// Archive information
    #[serde(default)]
    pub archive_info: ArchiveInfo,
    /// Deployment information, if any
    #[serde(default)]
    pub deploy_info: DeployInfo,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

fn default_id() -> Uuid {
    Uuid::new_v4()
}

#[allow(dead_code)]
fn is_none_or_empty<T: AsRef<str>>(s: &Option<T>) -> bool {
    match s {
        Some(value) => value.as_ref().is_empty(),
        None => true,
    }
}

/// Fields in [Metadata] with optional, used in communication with the frontend
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct MetadataOption {
    #[serde(default)]
    pub id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "is_none_or_empty")]
    pub title: Option<String>,
    #[serde(default)]
    pub alias: Option<Vec<String>>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub collection: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub content_info: Option<ContentInfo>,
    #[serde(default)]
    pub archive_info: Option<ArchiveInfo>,

    #[serde(default)]
    pub flag_create_archive: bool,
}

impl Metadata {
    pub async fn create(opt: MetadataOption) -> Result<Self> {
        let id = default_id();
        let time = Utc::now();
        info!("Creating metadata: {} at {}", id, time);
        let mut created = Self {
            title: opt.title.unwrap_or(format!("Unnamed {id}")),
            alias: opt.alias.unwrap_or_default(),
            tags: opt.tags.unwrap_or_default(),
            collection: opt.collection,
            description: opt.description,
            image: opt.image,
            content_info: opt.content_info.unwrap_or_default(),
            archive_info: opt.archive_info.clone().unwrap_or_default(),
            deploy_info: DeployInfo::None,
            create_time: time,
            update_time: time,
            id,
        };

        if opt.flag_create_archive {
            if let Some(archive_info) = opt.archive_info {
                if let ArchiveInfo::ArchiveFile { .. } = archive_info {
                    created.process_archive().await?;
                }
            } else {
                warn!(
                    "Set 'flag_create_archive' but no archive_info provided, skipping archive creation."
                );
            }
        }

        Ok(created)
    }

    pub async fn patch(&mut self, opt: MetadataOption) -> Result<()> {
        info!("Patching metadata: {} with {:?}", self.id, opt);
        if let Some(title) = opt.title {
            self.title = title;
        }
        if let Some(alias) = opt.alias {
            self.alias = alias;
        }
        if let Some(tags) = opt.tags {
            self.tags = tags;
        }
        if let Some(collection) = opt.collection {
            self.collection = Some(collection);
        }
        if let Some(description) = opt.description {
            self.description = Some(description);
        }
        if let Some(image) = opt.image {
            self.image = Some(image);
        }
        if let Some(content_info) = opt.content_info {
            self.content_info = content_info;
        }
        if let Some(archive_info) = opt.archive_info {
            if opt.flag_create_archive {
                if let ArchiveInfo::ArchiveFile { .. } = archive_info {
                    if let This(existing_path) = self.archive_info.try_resolve()? {
                        info!(
                            "Found existing archive file: {}, removing",
                            existing_path.display()
                        );
                        fs::remove_file(existing_path).unwrap_or_else(|e| {
                            warn!("Failed to remove existing archive file: {}", e);
                        });
                    }
                    self.archive_info = archive_info;
                    self.process_archive().await?;
                } else {
                    warn!(
                        "Set 'flag_create_archive' but archive_info is not ArchiveFile, skipping archive creation."
                    );
                }
            } else {
                self.archive_info = archive_info;
            }
        }

        let update_time = Utc::now();
        self.update_time = update_time;
        info!("Updating metadata: {} at {}", self.id, update_time);

        Ok(())
    }

    pub fn mark_update(&mut self) {
        self.update_time = Utc::now();
    }

    async fn process_archive(&mut self) -> Result<()> {
        let (raw_path, password) = match self.archive_info.clone() {
            ArchiveInfo::ArchiveFile { path, password, .. } => (path, password),
            _ => unreachable!(),
        };
        let raw_path = Path::new(&raw_path);
        if !raw_path.exists() {
            return Err(anyhow!(
                "Archive source path does not exist: {}",
                raw_path.display()
            ));
        }
        if !raw_path.is_dir() {
            return Err(anyhow!(
                "Archive source path is not a directory: {}",
                raw_path.display()
            ));
        }

        let dir_base = get_config_copy()?.dir_archive();
        let dir_rel = self.content_info.path_rel();

        let file_name = format!("{}.a", self.content_info.file_name());

        let mut target_path = dir_base.join(&dir_rel);
        tfs::create_dir_all(&target_path).await.map_err(|e| {
            anyhow!(
                "Failed to create directory {}: {}",
                target_path.display(),
                e
            )
        })?;
        target_path.push(&file_name);

        info!(
            "Processing archive: {}, saving to {}",
            file_name,
            target_path.display()
        );
        compress(raw_path, &target_path, password.as_deref())
            .await
            .map_err(|e| {
                error!("Failed to compress archive: {}", e);
                anyhow!("Failed to compress archive: {}", e)
            })?;

        self.archive_info = ArchiveInfo::ArchiveFile {
            size: target_path.calculate_size(),
            path: dir_rel.join(file_name).to_string_lossy().to_string(),
            password,
        };
        info!("Created archive for metadata: {}", self.id);

        Ok(())
    }

    pub async fn deploy(&mut self, target: impl AsRef<Path>) -> Result<bool> {
        let target_path = target.as_ref().to_owned();
        if !target_path.exists() {
            return Err(anyhow!(
                "Target path does not exist: {}",
                target_path.display()
            ));
        }
        if !target_path.is_dir() {
            return Err(anyhow!(
                "Target path is not a directory: {}",
                target_path.display()
            ));
        }

        match self.archive_info.try_resolve()? {
            This(source_path) => match &self.archive_info {
                ArchiveInfo::CommonFile { .. } => {
                    let target_file = target_path.join(
                        source_path
                            .file_name()
                            .unwrap_or_else(|| OsStr::new("Why?")),
                    );
                    info!(
                        "Deploying common file by copying from {} to {}",
                        source_path.display(),
                        target_file.display()
                    );

                    tfs::copy(&source_path, &target_file).await?;
                    self.deploy_info = DeployInfo::new_file(target_file.clone());
                    self.mark_update();

                    info!("Deployed common file to: {}", target_file.display());

                    Ok(true)
                },
                ArchiveInfo::ArchiveFile { password, .. } => {
                    if !target_path.is_dir_empty() {
                        error!("Target directory is not empty: {}", target_path.display());
                        return Err(anyhow!(
                            "Target directory is not empty: {}",
                            target_path.display()
                        ));
                    }

                    info!(
                        "Deploying archive file from {} to {}",
                        source_path.display(),
                        target_path.display()
                    );

                    decompress(source_path, &target_path, password.as_deref()).await?;

                    self.deploy_info = DeployInfo::new_dir(target_path.to_owned());
                    self.mark_update();

                    info!("Deployed archive to: {}", target_path.display());

                    Ok(true)
                },
                ArchiveInfo::Directory { .. } => {
                    if !target_path.is_dir_empty() {
                        error!("Target directory is not empty: {}", target_path.display());
                        return Err(anyhow!(
                            "Target directory is not empty: {}",
                            target_path.display()
                        ));
                    }

                    info!(
                        "Deploying directory from {} to {}",
                        source_path.display(),
                        target_path.display()
                    );

                    let target_clone = target_path.clone();
                    async_runtime::spawn_blocking(move || {
                        dir::copy(
                            source_path,
                            target_clone,
                            &CopyOptions::new().copy_inside(true).overwrite(true),
                        )
                    })
                    .await??;

                    self.deploy_info = DeployInfo::new_dir(target_path.to_owned());
                    self.mark_update();

                    info!("Deployed directory to: {}", target_path.display());

                    Ok(true)
                },
                _ => unreachable!(),
            },
            That(archive_info) => {
                warn!("Archive info is not resolved, cannot deploy",);
                self.archive_info = archive_info;
                Ok(false)
            },
        }
    }

    pub async fn deploy_off(&mut self) -> Result<bool> {
        let path = match self.deploy_info.try_resolve() {
            This(path) => path,
            That(update) => {
                error!("Deploy info is invalid or missing");
                self.deploy_info = update;
                return Ok(false);
            },
        };
        match self.deploy_info {
            DeployInfo::None => {
                warn!("Deploy info not exists, cannot deploy off.");
                Ok(false)
            },
            DeployInfo::File { .. } => {
                info!("Removing deployed file at: {}", path.display());
                tfs::remove_file(&path).await?;
                self.deploy_info = DeployInfo::None;
                self.mark_update();
                info!("File deployed off successfully.");
                Ok(true)
            },
            DeployInfo::Directory { .. } => {
                info!("Removing deployed directory at: {}", path.display());
                path.clear_dir()?;
                self.deploy_info = DeployInfo::None;
                self.mark_update();
                info!("Directory deployed off successfully.");
                Ok(true)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_content_info() {
        let c1 = ContentInfo::Undefined;
        let c1_rel = c1.path_rel();
        println!("{:?}", c1_rel);
        assert_eq!(c1_rel, PathBuf::from("undefined"));

        let c2 = ContentInfo::Game(GameData {
            version: "1.0".to_string(),
            developer: Some("Dev".to_string()),
            publisher: Some("Pub".to_string()),
            sys_platform: vec![GameSysPlatform::Windows],
            distribution: GameDistribution::Steam(SteamDistributionData {
                app_id: "123456".to_string(),
            }),
            game_type: GameType::RPG,
        });
        let c2_rel = c2.path_rel();
        println!("{:?}", c2_rel);
        assert_eq!(c2_rel, PathBuf::from("game/steam"));

        let base = PathBuf::from("any_base/this/is/for/test");
        let c2_full = base.join(c2_rel);
        println!("{:?}", c2_full);
        assert_eq!(
            c2_full,
            PathBuf::from("any_base/this/is/for/test/game/steam")
        );
    }
}
