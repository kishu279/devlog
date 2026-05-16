use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitPayload {
    pub hash: String,
    pub message: String,

    pub files_changed: Vec<String>,

    pub insertions: i64,
    pub deletions: i64,

    pub branch: String,
}
