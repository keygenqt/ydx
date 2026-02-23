CREATE TYPE sort_cards AS ENUM ('random', 'alphabetical', 'date_added');

CREATE TABLE sets (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            VARCHAR(255) NOT NULL,
    icon            TEXT,
    description     TEXT,
    known_limit     INTEGER NOT NULL DEFAULT 10,
    session_limit   INTEGER NOT NULL DEFAULT 20,
    sort_by         sort_cards NOT NULL DEFAULT 'date_added',
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sets_user_id ON sets(user_id);
CREATE INDEX idx_sets_name ON sets(name);
CREATE INDEX idx_sets_created_at ON sets(created_at);
