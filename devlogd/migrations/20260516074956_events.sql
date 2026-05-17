-- Add migration script here
CREATE TABLE events (
    id INTEGER PRIMARY KEY,

    kind TEXT NOT NULL,
    ts INTEGER NOT NULL,

    project TEXT NOT NULL,

    payload TEXT NOT NULL
);