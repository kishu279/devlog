use anyhow::Context;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::path::PathBuf;

pub async fn connect_db() -> anyhow::Result<SqlitePool> {
    let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("failed to resolve workspace root")?
        .join("demo")
        .join("my_connection.db");

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create database directory {}", parent.display()))?;
    }

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .with_context(|| format!("failed to open database {}", db_path.display()))?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run database migrations")?;

    Ok(pool)
}

// use sqlx::migrate::Migrator;
// pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
//     // This reads ALL .sql files from src/migrations/ folder
//     // and runs them in order
//     Migrator::new(std::path::Path::new("src/migrations"))
//         .await?
//         .run(pool)
//         .await?;
//     Ok(())
// }

// CREATE INDEX idx_events_project_ts
// ON events(project, ts DESC);

// CREATE INDEX idx_events_kind_ts
// ON events(kind, ts DESC);

// CREATE INDEX idx_events_project_kind_ts
// ON events(project, kind, ts DESC);

// CREATE TABLE events (
//     id INTEGER PRIMARY KEY,

//     kind TEXT NOT NULL,
//     ts INTEGER NOT NULL,

//     project TEXT NOT NULL,

//     payload TEXT NOT NULL
// );
