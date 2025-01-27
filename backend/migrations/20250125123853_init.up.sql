-- Add up migration script here
-- Enable UUID support
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Users table with login codes
CREATE TABLE IF NOT EXISTS "user" (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    username varchar(255) NOT NULL UNIQUE,
    code varchar(255) NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Scores table with detailed game stats
CREATE TABLE IF NOT EXISTS score (
    id bigserial PRIMARY KEY,
    user_id uuid NOT NULL,
    score integer NOT NULL,
    floor_reached integer NOT NULL,
    play_time_seconds integer NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "user" (id)
);

-- Indexes for performance
CREATE INDEX idx_user_login ON "user" (code);

CREATE INDEX idx_scores_user_score ON score (user_id, score DESC);

