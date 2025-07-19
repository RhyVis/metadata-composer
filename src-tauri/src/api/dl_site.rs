use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Types of content available on the DLSite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum DLContentType {
    /// All Ages; Games, Voice / ASMR, Manga / CG;
    /// 同人
    Doujin,
    /// All Ages; Shonen/Seinen Comics, Shojo/Josei Comics, Webtoon, Voiced Comics, Light Novels, Novels, Generals;
    /// コミック（comipo）
    Comics,
    /// All Ages; Games, Voice Dramas / ASMR, Music;
    /// PCソフト
    PcGames,
    /// All Ages; Android Apps, Games;
    /// スマホゲーム
    SmartphoneGames,
    /// R18; Games, Voice/ASMR, Manga/CG, Webtoon, Voiced Comics
    /// 同人
    DoujinR18,
    /// R18; Books, Magazines/Anthologies, Short Stories, Webtoon, Voiced Comics
    /// 成年コミック
    ComicsR18,
    /// R18; Games, Anime, Voice Dramas / ASMR / Music
    /// 美少女ゲーム
    HGames,
    /// R18; Android Apps, Games
    /// スマホゲーム
    SmartphoneGamesR18,
}

#[allow(dead_code)]
impl DLContentType {
    pub fn name_url(&self) -> &'static str {
        match self {
            DLContentType::Doujin => "home",
            DLContentType::Comics => "comic",
            DLContentType::PcGames => "soft",
            DLContentType::SmartphoneGames => "app",
            DLContentType::DoujinR18 => "maniax",
            DLContentType::ComicsR18 => "books",
            DLContentType::HGames => "pro",
            DLContentType::SmartphoneGamesR18 => "appx",
        }
    }

    pub fn name_prefix(&self) -> &'static str {
        match self {
            DLContentType::Doujin
            | DLContentType::DoujinR18
            | DLContentType::SmartphoneGames
            | DLContentType::SmartphoneGamesR18 => "RJ",
            DLContentType::Comics | DLContentType::ComicsR18 => "BJ",
            DLContentType::PcGames | DLContentType::HGames => "VJ",
        }
    }
}
