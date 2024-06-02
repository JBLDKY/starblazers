-- Add migration script here
CREATE TABLE IF NOT EXISTS players (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) NOT NULL,
    games_played INTEGER DEFAULT 0
);
