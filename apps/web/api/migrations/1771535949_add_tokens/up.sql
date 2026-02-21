CREATE TABLE tokens (
    id                  SERIAL PRIMARY KEY,
    user_id             INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    hash_refresh_token  TEXT NOT NULL,
    device_uuid         TEXT NOT NULL,
    expires_at          TIMESTAMP NOT NULL,
    created_at          TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_tokens_user_device ON tokens(user_id, device_uuid);
CREATE INDEX idx_hash_refresh_token ON tokens(hash_refresh_token);
CREATE INDEX idx_tokens_expires_at ON tokens(expires_at);
