use crate::event::{kind::EventKind, payload::EventPayload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DevlogEvent {
    pub kind: EventKind,
    pub ts: i64,

    pub project: String,

    pub payload: EventPayload,
}
