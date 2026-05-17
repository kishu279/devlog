-- Add migration script here
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    name TEXT NOT NULL,

    path TEXT NOT NULL UNIQUE,

    path_hash TEXT NOT NULL UNIQUE,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_projects_name
ON projects(name);

CREATE INDEX idx_projects_hash
ON projects(path_hash);