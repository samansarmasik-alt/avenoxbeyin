use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Internal,
    Private,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Internal
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Inbox,
    Active,
    Waiting,
    Blocked,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub source: String,
    pub text: String,
    pub source_sha256: String,
    #[serde(default = "default_revision")]
    pub revision: u64,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub updated_at: Option<String>,
    #[serde(default)]
    pub supersedes: Vec<String>,
    #[serde(default)]
    pub facts: HashMap<String, serde_json::Value>,
}

fn default_revision() -> u64 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub event_id: String,
    pub summary: String,
    pub refs: Vec<String>,
    pub harness: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub session: Option<String>,
    #[serde(default)]
    pub links: Vec<String>,
}
