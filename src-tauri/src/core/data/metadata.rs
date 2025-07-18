use crate::api::dl_site::DLContentType;
use crate::core::Whether;
use crate::core::Whether::{That, This};
use crate::core::util::compress::{compress, decompress};
use crate::core::util::config::get_config_copy;
use crate::core::util::path_ext::PathExt;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use fs_extra::dir;
use fs_extra::dir::CopyOptions;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use ts_rs::TS;
use uuid::Uuid;

/// Represents the type of content for a data item, with detailed information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum ContentInfo {
    #[default]
    Undefined,

    Game(GameData),
}

impl ContentInfo {
    fn dir_name(&self) -> Vec<&'static str> {
        match self {
            ContentInfo::Undefined => vec!["undefined"],
            ContentInfo::Game(data) => vec!["game", data.distribution.dir_name()],
        }
    }

    /// Returns the relative path starting from `.`, but it **should** start from `config.dir_archive()`
    ///
    /// This is used to store in metadata
    fn path_rel(&self) -> PathBuf {
        let mut base = PathBuf::new();
        for dir in self.dir_name() {
            base.push(dir);
        }
        base
    }

    fn file_name(&self) -> String {
        match self {
            ContentInfo::Undefined => {
                format!("UndefinedContent-{}", Utc::now().format("%Y%m%d%H%M%S"))
            }
            ContentInfo::Game(data) => data.distribution.file_name(),
        }
    }
}

/// Represents game data, including version, developer, publisher, and platform information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct GameData {
    #[serde(default = "default_version")]
    pub version: String,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub sys_platform: Vec<GameSysPlatform>,
    pub distribution: GameDistribution,
}

/// Represents the platform on which a game can run
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
#[allow(clippy::upper_case_acronyms)]
pub enum GameSysPlatform {
    Windows,
    Linux,
    MacOS,
    Android,
    IOS,
    Web,
}

/// Represents the distribution method of a game, such as Steam or DLSite
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum GameDistribution {
    #[default]
    Unknown,
    Steam {
        app_id: u32,
    },
    DLSite {
        id: String,
        content_type: DLContentType,
    },
}

impl GameDistribution {
    fn dir_name(&self) -> &'static str {
        match self {
            GameDistribution::Unknown => "unknown",
            GameDistribution::Steam { .. } => "steam",
            GameDistribution::DLSite { .. } => "dl",
        }
    }

    fn file_name(&self) -> String {
        match self {
            Self::Unknown => format!("Unknown-{}", Utc::now().format("%Y%m%d%H%M%S")),
            Self::Steam { app_id } => app_id.to_string(),
            Self::DLSite { id, .. } => id.to_string(),
        }
    }
}

/// Represents archive information for a data item, such as size and path
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, Default)]
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
    fn try_resolve(&self) -> Result<Whether<PathBuf, Self>> {
        match self {
            ArchiveInfo::None => {
                warn!("Trying to resolve None archive info, returning self");
                Ok(That(ArchiveInfo::None))
            }
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
            }
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
            }
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
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, Default)]
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
    fn new_file(path: PathBuf) -> Self {
        DeployInfo::File { path }
    }

    fn new_dir(path: PathBuf) -> Self {
        DeployInfo::Directory { path }
    }

    fn try_resolve(&self) -> Whether<PathBuf, DeployInfo> {
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

/// Basic metadata structure for data item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct Metadata {
    /// Unique identifier for the data item,
    /// by default a [Uuid]
    pub id: Uuid,

    /// The title of the data item
    pub title: String,
    /// The other names of the data item
    pub alias: Vec<String>,
    /// Tags associated with the data item
    pub tags: Vec<String>,
    /// Collection names, if any
    pub collection: Option<String>,
    /// Description of the data item
    pub description: Option<String>,
    /// Image hash, if any
    pub image: Option<String>,

    /// The content type of the data item
    pub content_info: ContentInfo,
    /// Archive information
    pub archive_info: ArchiveInfo,
    /// Deployment information, if any
    pub deploy_info: DeployInfo,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

fn default_id() -> Uuid {
    Uuid::new_v4()
}

fn default_version() -> String {
    String::from("1.0.0")
}

/// Fields in [Metadata] with optional, used in communication with the frontend
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct MetadataOption {
    #[serde(default)]
    pub id: Option<Uuid>,
    #[serde(default)]
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
    pub fn create(opt: MetadataOption) -> Result<Self> {
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
                    created.process_archive()?;
                }
            } else {
                warn!(
                    "Set 'flag_create_archive' but no archive_info provided, skipping archive creation."
                );
            }
        }

        Ok(created)
    }

    pub fn patch(&mut self, opt: MetadataOption) {
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
            self.archive_info = archive_info;
        }
        let update_time = Utc::now();
        self.update_time = update_time;
        info!("Updating metadata: {} at {}", self.id, update_time);
    }

    pub fn mark_update(&mut self) {
        self.update_time = Utc::now();
    }

    fn process_archive(&mut self) -> Result<()> {
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
        fs::create_dir_all(&target_path)?;
        target_path.push(&file_name);

        info!(
            "Processing archive: {}, saving to {}",
            file_name,
            target_path.display()
        );
        compress(raw_path, &target_path, password.as_deref(), Some(9))?;

        self.archive_info = ArchiveInfo::ArchiveFile {
            size: target_path.calculate_size(),
            path: dir_rel.join(file_name).to_string_lossy().to_string(),
            password,
        };
        info!("Created archive for metadata: {}", self.id);

        Ok(())
    }

    pub fn deploy(&mut self, target: impl AsRef<Path>) -> Result<bool> {
        let target_path = target.as_ref();
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

                    fs::copy(&source_path, &target_file)?;
                    self.deploy_info = DeployInfo::new_file(target_file.clone());
                    self.mark_update();

                    info!("Deployed common file to: {}", target_file.display());

                    Ok(true)
                }
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

                    decompress(source_path, target_path, password.as_deref())?;

                    self.deploy_info = DeployInfo::new_dir(target_path.to_owned());
                    self.mark_update();

                    info!("Deployed archive to: {}", target_path.display());

                    Ok(true)
                }
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

                    dir::copy(
                        source_path,
                        target_path,
                        &CopyOptions::new().copy_inside(true).overwrite(true),
                    )?;

                    self.deploy_info = DeployInfo::new_dir(target_path.to_owned());
                    self.mark_update();

                    info!("Deployed directory to: {}", target_path.display());

                    Ok(true)
                }
                _ => unreachable!(),
            },
            That(archive_info) => {
                warn!("Archive info is not resolved, cannot deploy",);
                self.archive_info = archive_info;
                Ok(false)
            }
        }
    }

    pub fn deploy_off(&mut self) -> Result<bool> {
        let path = match self.deploy_info.try_resolve() {
            This(path) => path,
            That(update) => {
                error!("Deploy info is invalid or missing");
                self.deploy_info = update;
                return Ok(false);
            }
        };
        match self.deploy_info {
            DeployInfo::None => {
                warn!("Deploy info not exists, cannot deploy off.");
                Ok(false)
            }
            DeployInfo::File { .. } => {
                info!("Removing deployed file at: {}", path.display());
                fs::remove_file(&path)?;
                self.deploy_info = DeployInfo::None;
                self.mark_update();
                info!("File deployed off successfully.");
                Ok(true)
            }
            DeployInfo::Directory { .. } => {
                info!("Removing deployed directory at: {}", path.display());
                path.clear_dir()?;
                self.deploy_info = DeployInfo::None;
                self.mark_update();
                info!("Directory deployed off successfully.");
                Ok(true)
            }
        }
    }
}

#[cfg(test)]
mod test {
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
            distribution: GameDistribution::Steam { app_id: 12345 },
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
