// Functions regarding stored events:
// insert_event
// get_events
// get_events_by_day
// get_events_by_project

use crate::{
    event::{
        event::DevlogEvent,
        kind::EventKind,
        payload::{commit::CommitPayload, EventPayload},
    },
    store::row::EventRow,
};
use sqlx::SqlitePool;

pub async fn insert_event(
    event: &DevlogEvent,
    pool: &SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
    let payload_json = serde_json::to_string(&event.payload)?;



    // println!("{}", );

    let event_key = event.event_key();
    println!("{}", event_key);

    let output = sqlx::query(
        r#"
        INSERT OR IGNORE INTO events(
            event_key,
            kind,
            ts,
            project,
            payload
        )
        VALUES (?, ?, ?, ?, ?)
    "#,
    )
    .bind(event_key)
    .bind(&event.kind)
    .bind(&event.ts)
    .bind(&event.project)
    .bind(payload_json)
    .execute(pool)
    .await?;

    println!("{}", output.rows_affected());

    Ok(())
}
