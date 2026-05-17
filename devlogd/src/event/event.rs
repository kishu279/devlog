use crate::event::{kind::EventKind, payload::EventPayload};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DevlogEvent {
    pub kind: EventKind,
    pub ts: i64,

    pub project: String,

    pub payload: EventPayload,
}

impl DevlogEvent {
    pub fn event_key(&self) -> String {
        let event = self;
        let event_key = match &event.kind {
            EventKind::Commit => {
                // take the hash and the timestamp
                let hash = match &event.payload {
                    EventPayload::Commit(payload) => &payload.hash,
                    _ => panic!("Expected commit payload"),
                };

                let ts = event.ts;

                format!("commit:{}:{}", hash, ts)
            }
            EventKind::FileChange | EventKind::ShellCommand | EventKind::EditorActivity => {
                todo!("Will implement this by today")
            }
        };

        return event_key;
    }
}
