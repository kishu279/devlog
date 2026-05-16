pub mod commit;

use commit::CommitPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventPayload {
    Commit(CommitPayload),
}
