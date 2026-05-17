use sqlx::FromRow;

// EVENT STRUCT for database operations
#[derive(Debug, FromRow)]
pub struct EventRow {
    pub id: i64,

    pub kind: String,
    pub ts: i64,

    pub project: String,

    pub payload: String,
}

#[derive(Debug, FromRow)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub path_hash: String,
}
