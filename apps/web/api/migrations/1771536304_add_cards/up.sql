CREATE TABLE cards (
    id          SERIAL PRIMARY KEY,
    set_id      INTEGER NOT NULL REFERENCES sets(id) ON DELETE CASCADE,
    term        TEXT NOT NULL,
    definition  TEXT NOT NULL,
    image       TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_cards_set_id ON cards(set_id);
