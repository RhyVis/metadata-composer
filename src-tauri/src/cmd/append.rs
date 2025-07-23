use serde::Deserialize;
use ts_rs::TS;

use crate::api::dl_site::DLContentType;

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct DeployArg {
    pub use_config_dir: bool,
    pub target_dir: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct DLFetchArg {
    pub id: String,
    pub content_type: DLContentType,
}
