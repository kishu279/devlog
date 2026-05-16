use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventKind {
    Commit,
    FileChange,
    ShellCommand,
    EditorActivity,
}
