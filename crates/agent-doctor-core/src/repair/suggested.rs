use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedRepair {
    pub id: String,
    pub title: String,
    pub description: String,
    pub auto_fixable: bool,
}
