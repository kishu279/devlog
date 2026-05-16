use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

// CONNECTION POOL
pub async fn connect_db() -> anyhow::Result<SqlitePool> {
    let connection_string = "sqlite://./../demo/my_connection.db";

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
