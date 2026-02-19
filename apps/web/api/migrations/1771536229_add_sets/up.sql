CREATE TABLE sets (
    id              SERIAL PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name            VARCHAR(255) NOT NULL,
    icon            TEXT,
    description     TEXT,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sets_user_id ON sets(user_id);
CREATE INDEX idx_sets_name ON sets(name);
CREATE INDEX idx_sets_created_at ON sets(created_at);
