use crate::api::dl_site::DLContentType;
use chrono::{DateTime, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

/// Represents collection information for grouping data items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct Collection {
    pub id: String,
    pub name: String,
}

/// Represents the type of content for a data item, with detailed information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum ContentInfo {
    #[default]
    Undefined,

    Game(GameData),
}

/// Represents game data, including version, developer, publisher, and platform information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct GameData {
    pub version: String,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub sys_platform: Vec<GameSysPlatform>,
    pub distribution: GameDistribution,
}

/// Represents the platform on which a game can run
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
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

impl Default for GameData {
    fn default() -> Self {
        GameData {
            version: default_version(),
            developer: None,
            publisher: None,
            sys_platform: Vec::new(),
            distribution: GameDistribution::Unknown,
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

/// Basic metadata structure for data item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct Metadata {
    /// Unique identifier for the data item,
    /// by default a [Uuid]
    pub id: String,

    /// The title of the data item
    pub title: String,
    /// The other names of the data item
    pub alias: Vec<String>,
    /// Tags associated with the data item
    pub tags: Vec<String>,
    /// Collection information, if any
    pub collection: Option<Collection>,

    /// The content type of the data item
    pub content_info: ContentInfo,
    /// Archive information
    pub archive_info: ArchiveInfo,

    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

fn default_id() -> String {
    Uuid::new_v4().to_string()
}

fn default_version() -> String {
    String::from("1.0.0")
}

/// Fields in [Metadata] with optional, used in communication with the frontend
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct MetadataOption {
    pub id: Option<String>,
    pub title: Option<String>,
    pub alias: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub collection: Option<Collection>,
    pub content_type: Option<ContentInfo>,
    pub archive_info: Option<ArchiveInfo>,
}

impl Metadata {
    pub fn create(opt: MetadataOption) -> Self {
        let id = default_id();
        let time = Utc::now();
        info!("Creating metadata: {} at {}", id, time);
        Self {
            title: opt.title.unwrap_or(format!("Unnamed {id}")),
            alias: opt.alias.unwrap_or_default(),
            tags: opt.tags.unwrap_or_default(),
            collection: None,
            content_info: opt.content_type.unwrap_or_default(),
            archive_info: opt.archive_info.unwrap_or_default(),
            create_time: time.clone(),
            update_time: time,
            id,
        }
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
        if let Some(content_type) = opt.content_type {
            self.content_info = content_type;
        }
        if let Some(archive_info) = opt.archive_info {
            self.archive_info = archive_info;
        }
        let update_time = Utc::now();
        info!("Updating metadata: {} at {}", self.id, update_time);
        self.update_time = update_time;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn content_type() {
        #[derive(Debug, Serialize)]
        struct Wrapper {
            content: ContentInfo,
        }

        let wrapper = Wrapper {
            content: ContentInfo::Game(GameData::default()),
        };
        let serialized = serde_json::to_string_pretty(&wrapper).unwrap();

        dbg!(&wrapper);
        println!("{}", serialized);
    }
}
