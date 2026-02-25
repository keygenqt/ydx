CREATE TYPE entity_type_image AS ENUM ('user', 'set', 'card');

CREATE TABLE images (
    id              SERIAL PRIMARY KEY,
    entity_type     entity_type_image NOT NULL,
    entity_id       INTEGER NOT NULL,
    path            TEXT NOT NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_images_entity ON images(entity_type, entity_id);
