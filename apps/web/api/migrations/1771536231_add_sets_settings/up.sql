CREATE TYPE sort_order AS ENUM ('random', 'alphabetical', 'date_added');

CREATE TABLE sets_settings (
    id              SERIAL PRIMARY KEY,
    set_id          INTEGER NOT NULL REFERENCES sets(id) ON DELETE CASCADE UNIQUE,
    known_limit     INTEGER DEFAULT 10,
    session_limit   INTEGER DEFAULT 20,
    sort_by         sort_order DEFAULT 'date_added'
);

CREATE INDEX idx_sets_settings_set_id ON sets_settings(set_id);
