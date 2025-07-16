use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Types of content available on the DLSite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub enum DLContentType {
    /// 同人; R18 同人游戏
    Maniax,
    /// 成年コミック; R18 成人漫画
    Books,
    /// 美少女ゲーム; R18 美少女游戏
    Pro,
    /// スマホゲーム; R18 手机游戏
    AppX,
}
