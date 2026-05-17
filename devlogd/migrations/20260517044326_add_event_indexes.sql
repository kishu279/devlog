-- Add migration script here
CREATE INDEX idx_events_project_ts
ON events(project, ts DESC);

CREATE INDEX idx_events_kind_ts
ON events(kind, ts DESC);

CREATE INDEX idx_events_project_kind_ts
ON events(project, kind, ts DESC);