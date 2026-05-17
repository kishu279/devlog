use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EventKind {
    Commit,
    FileChange,
    ShellCommand,
    EditorActivity,
}

// impl EventKind {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             EventKind::Commit => "commit",
//             EventKind::FileChange => "file_change",
//             EventKind::ShellCommand => "shell_command",
//             EventKind::EditorActivity => "editor_activity",
//         }
//     }
// }

// let kind = match row.kind.as_str() {
//     "commit" => EventKind::Commit,
//     "file_change" => EventKind::FileChange,
//     "shell_command" => EventKind::ShellCommand,
//     "editor_activity" => EventKind::EditorActivity,
//     _ => return Err(...),
// };
