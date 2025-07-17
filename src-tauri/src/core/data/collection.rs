use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

/// Represents collection information for grouping data items
#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../src/api/types.ts")]
pub struct CollectionDef {
    pub id: Uuid,
    pub name: String,
}
