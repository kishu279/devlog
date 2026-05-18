use std::path::PathBuf;

use chrono::Timelike;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    FromRow,
};

pub use sqlx::SqlitePool;

pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .expect("cannot find home dir")
        .join(".devlog")
        .join("events.db")
}

pub async fn get_pool() -> sqlx::Result<SqlitePool> {
    let db_path = db_path();

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: i64,
    pub ts: i64,
    pub kind: String,
    pub project: String,
    pub payload: String,
}

pub async fn get_commits_today(pool: &SqlitePool) -> sqlx::Result<Vec<Event>> {
    let midnight = today_midnight_ts();
    sqlx::query_as::<_, Event>(
        r#"
        SELECT id, ts, kind, project, payload
        FROM events
        WHERE kind IN ('Commit', 'commit')
          AND ts >= ?
        ORDER BY ts ASC
        "#,
    )
    .bind(midnight)
    .fetch_all(pool)
    .await
}

pub async fn get_files_today(pool: &SqlitePool) -> sqlx::Result<Vec<Event>> {
    let midnight = today_midnight_ts();
    sqlx::query_as::<_, Event>(
        r#"
        SELECT id, ts, kind, project, payload
        FROM events
        WHERE kind IN ('FileChange', 'file_change', 'file')
          AND ts >= ?
        ORDER BY ts ASC
        "#,
    )
    .bind(midnight)
    .fetch_all(pool)
    .await
}

pub async fn get_cmds_today(pool: &SqlitePool) -> sqlx::Result<Vec<Event>> {
    let midnight = today_midnight_ts();
    sqlx::query_as::<_, Event>(
        r#"
        SELECT id, ts, kind, project, payload
        FROM events
        WHERE kind IN ('ShellCommand', 'shell_command', 'cmd')
          AND ts >= ?
        ORDER BY ts ASC
        "#,
    )
    .bind(midnight)
    .fetch_all(pool)
    .await
}

pub fn today_midnight_ts() -> i64 {
    let now = chrono::Local::now();
    now.timestamp() - i64::from(now.time().num_seconds_from_midnight())
}
