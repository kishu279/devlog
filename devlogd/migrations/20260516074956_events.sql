-- Add migration script here
CREATE TABLE events (
    id INTEGER PRIMARY KEY,

    kind TEXT NOT NULL,
    ts INTEGER NOT NULL,

    project TEXT NOT NULL,

    payload TEXT NOT NULL
);

CREATE INDEX idx_events_project_ts
ON events(project, ts DESC);

CREATE INDEX idx_events_kind_ts
ON events(kind, ts DESC);

CREATE INDEX idx_events_project_kind_ts
ON events(project, kind, ts DESC);