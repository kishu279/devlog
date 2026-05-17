-- Add migration script here
ALTER TABLE events
ADD COLUMN event_key TEXT NOT NULL DEFAULT '';

CREATE UNIQUE INDEX idx_events_event_key
ON events(event_key);