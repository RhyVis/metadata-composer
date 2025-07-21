use crate::api::get_client;
use crate::core::Language;
use anyhow::{Result, anyhow};
use log::{info, warn};
use scraper::error::SelectorErrorKind;
use scraper::{Html, Selector};
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

impl DLContentType {
    pub const fn name_url(&self) -> &'static str {
        match self {
            Self::Doujin => "home",
            Self::Comics => "comic",
            Self::PcGames => "soft",
            Self::SmartphoneGames => "app",
            Self::DoujinR18 => "maniax",
            Self::ComicsR18 => "books",
            Self::HGames => "pro",
            Self::SmartphoneGamesR18 => "appx",
        }
    }

    pub const fn name_prefix(&self) -> &'static str {
        match self {
            Self::Doujin | Self::DoujinR18 | Self::SmartphoneGames | Self::SmartphoneGamesR18 => {
                "RJ"
            }
            Self::Comics | Self::ComicsR18 => "BJ",
            Self::PcGames | Self::HGames => "VJ",
        }
    }

    pub fn build_id(&self, id: &str) -> String {
        format!("{}{}", self.name_prefix(), id)
    }
}

trait DLSiteLang {
    fn dl_lang_code(&self) -> &'static str;
    fn dl_tag_scenario(&self) -> &'static str;
    fn dl_tag_illustration(&self) -> &'static str;
}

impl DLSiteLang for Language {
    fn dl_lang_code(&self) -> &'static str {
        match self {
            Self::EnUs => "en_US",
            Self::ZhCn => "zh_CN",
            Self::JaJp => "ja_JP",
        }
    }
    fn dl_tag_scenario(&self) -> &'static str {
        match self {
            Self::EnUs => "Scenario",
            Self::ZhCn => "剧情",
            Self::JaJp => "シナリオ",
        }
    }
    fn dl_tag_illustration(&self) -> &'static str {
        match self {
            Self::EnUs => "Illustration",
            Self::ZhCn => "插画",
            Self::JaJp => "イラスト",
        }
    }
}

#[derive(Debug, Clone, Serialize, Default, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct DLFetchInfo {
    pub title: String,
    pub circle: String,
    pub scenario: Vec<String>,
    pub illustration: Vec<String>,
    pub category: Vec<String>,
    pub tags: Vec<String>,
    pub description: Vec<String>,
}

pub trait DLContentFetch {
    fn fetch_info(
        &self,
        id: &str,
        lang: &Language,
    ) -> impl Future<Output = Result<DLFetchInfo>> + Send;
}

impl DLContentFetch for DLContentType {
    async fn fetch_info(&self, id: &str, lang: &Language) -> Result<DLFetchInfo> {
        let client = get_client();
        let url = build_url(self, &self.build_id(id), lang);

        info!("Requesting {url} for content type: {:?}", self);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Could not fetch content type for {}: {}", url, e))?;
        let document = Html::parse_document(
            &response
                .text()
                .await
                .map_err(|e| anyhow!("Failed to parse response text for {}: {}", url, e))?,
        );

        let title_selector = Selector::parse("#work_name").to_anyhow()?;
        let title = document
            .select(&title_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|text| text.trim().to_string())
            .unwrap_or_else(|| {
                warn!("Title not found for URL: {}", url);
                String::new()
            });

        let circle_selector = Selector::parse(".maker_name a").to_anyhow()?;
        let circle = document
            .select(&circle_selector)
            .next()
            .and_then(|el| el.text().next())
            .map(|text| text.trim().to_string())
            .unwrap_or_else(|| {
                warn!("Circle not found for URL: {}", url);
                String::new()
            });

        let tr_selector = Selector::parse("tr").to_anyhow()?;

        let mut scenario = Vec::new();
        let mut illustration = Vec::new();
        let mut flag = (false, false);

        for tr in document.select(&tr_selector) {
            let th = tr.select(&Selector::parse("th").to_anyhow()?).next();
            if let Some(th) = th {
                if th.text().collect::<String>().trim() == lang.dl_tag_scenario() {
                    scenario = tr
                        .select(&Selector::parse("td a").to_anyhow()?)
                        .map(|a| a.text().collect::<String>().trim().to_string())
                        .collect();
                    flag.0 = true;
                }
            }
            if let Some(th) = th {
                if th.text().collect::<String>().trim() == lang.dl_tag_illustration() {
                    illustration = tr
                        .select(&Selector::parse("td a").to_anyhow()?)
                        .map(|a| a.text().collect::<String>().trim().to_string())
                        .collect();
                    flag.0 = true;
                }
            }
            if flag.0 && flag.1 {
                break; // Both scenario and illustration found, exit loop
            }
        }

        let category_selector = Selector::parse("#category_type a").to_anyhow()?;
        let category = document
            .select(&category_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect::<Vec<_>>();

        let tag_selector = Selector::parse(".main_genre a").to_anyhow()?;
        let tags = document
            .select(&tag_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .collect::<Vec<_>>();

        let desc_selector =
            Selector::parse(".work_parts_container[itemprop=description]").to_anyhow()?;
        let description = document
            .select(&desc_selector)
            .flat_map(|el| el.text())
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect::<Vec<_>>();

        Ok(DLFetchInfo {
            title,
            circle,
            scenario,
            illustration,
            category,
            tags,
            description,
        })
    }
}

fn build_url(content_type: &DLContentType, id: &str, lang: &Language) -> String {
    format!(
        "https://www.dlsite.com/{}/work/=/product_id/{}.html/?locale={}",
        content_type.name_url(),
        id,
        lang.dl_lang_code()
    )
}

trait SelectorExt {
    fn to_anyhow(self) -> Result<Selector>;
}

impl SelectorExt for Result<Selector, SelectorErrorKind<'_>> {
    fn to_anyhow(self) -> Result<Selector> {
        self.map_err(|e| anyhow!("Failed to parse selector: {}", e))
    }
}

#[cfg(test)]
mod test {
    use crate::api::dl_site::{DLContentFetch, DLContentType};
    use crate::api::init_client;
    use crate::core::Language;

    fn init() {
        init_client().expect("Failed to initialize HTTP client for tests");
    }

    #[tokio::test]
    async fn test_fetch() {
        init();

        let id1 = "01239331"; // クルセイダー・プリンセス～闇に堕ちるココロとカラダ～
        let result1 = DLContentType::DoujinR18
            .fetch_info(id1, &Language::JaJp)
            .await
            .expect("fetch dl site maniax");
        dbg!(result1);

        let test2 = "01397047"; // 救出のお嬢様
        let result2 = DLContentType::DoujinR18
            .fetch_info(test2, &Language::ZhCn)
            .await
            .expect("fetch dl site maniax");
        dbg!(result2);

        let faulty_id = "00000000"; // Invalid ID for testing error handling
        let result3 = DLContentType::DoujinR18
            .fetch_info(faulty_id, &Language::EnUs)
            .await
            .expect("fetch dl site maniax");
        dbg!(result3);
    }
}
