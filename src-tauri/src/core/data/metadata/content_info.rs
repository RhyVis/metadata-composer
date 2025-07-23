use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::api::dl_site::DLContentType;

/// Represents the type of content for a data item, with detailed information
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum ContentInfo {
    #[default]
    Undefined,

    Game(GameData),
}

impl ContentInfo {
    fn dir_name(&self) -> Vec<&'static str> {
        match self {
            Self::Undefined => vec!["undefined"],
            Self::Game(data) => vec!["game", data.distribution.dir_name()],
        }
    }

    /// Returns the relative path starting from `.`, but it **should** start
    /// from `config.dir_archive()`
    ///
    /// This is used to store in metadata
    pub(super) fn path_rel(&self) -> PathBuf {
        let mut base = PathBuf::new();
        for dir in self.dir_name() {
            base.push(dir);
        }
        base
    }

    pub(super) fn file_name(&self) -> String {
        match self {
            Self::Undefined => format!("Content-{}", Utc::now().format("%Y%m%d%H%M%S")),
            Self::Game(data) => data.distribution.file_name(),
        }
    }
}

/// Represents game data, including version, developer, publisher, and platform
/// information
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct GameData {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub game_type: GameType,
    #[serde(default)]
    pub developer: Option<String>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub sys_platform: Vec<GameSysPlatform>,
    #[serde(default)]
    pub distribution: GameDistribution,
}

/// Represents the type of game, such as RPG Maker
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum GameType {
    #[default]
    Unspecified,
    RPG,
    SLG,
    AVG,
}

/// Represents the platform on which a game can run
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
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
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS, Default)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum GameDistribution {
    #[default]
    Unknown,
    Steam(SteamDistributionData),
    DLSite(DLSiteDistributionData),
    Other(OtherDistributionData),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct SteamDistributionData {
    pub app_id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct DLSiteDistributionData {
    pub id: String,
    pub content_type: DLContentType,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct OtherDistributionData {
    pub name: String,
    pub id: String,
}

impl DLSiteDistributionData {
    fn full_id(&self) -> String {
        self.content_type.build_id(&self.id)
    }
}

impl GameDistribution {
    fn dir_name(&self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Steam(_) => "steam",
            Self::DLSite(_) => "dl",
            Self::Other(_) => "other",
        }
    }

    fn file_name(&self) -> String {
        match self {
            Self::Unknown => format!("Unknown-{}", Utc::now().format("%Y%m%d%H%M%S")),
            Self::Steam(data) => data.app_id.to_string(),
            Self::DLSite(data) => data.full_id(),
            Self::Other(data) => format!("{}-{}", data.name, data.id),
        }
    }
}

fn default_version() -> String {
    String::from("1.0.0")
}
