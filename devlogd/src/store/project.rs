use sqlx::SqlitePool;

use crate::{helper::hash_path, store::row::Project};

pub async fn insert_project(pool: &SqlitePool, name: &str, path: &str) -> Result<(), sqlx::Error> {
    let path_hash = hash_path(path);

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO projects (
            name,
            path,
            path_hash
        )
        VALUES (?, ?, ?)
        "#,
    )
    .bind(name)
    .bind(path)
    .bind(path_hash)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_project(pool: &SqlitePool, name: &str) -> Result<Project, sqlx::Error> {
    let project = sqlx::query_as::<_, Project>(
        r#"
        SELECT name, path, path_hash
        FROM projects
        WHERE name = ?
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(project)
}

pub async fn get_all_projects(pool: &SqlitePool) -> Result<Vec<Project>, sqlx::Error> {
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT name, path, path_hash
        FROM projects
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}
