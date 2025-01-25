-- Add up migration script here
-- Users table with login codes
CREATE TABLE IF NOT EXISTS users (
    id integer PRIMARY KEY AUTOINCREMENT,
    username text NOT NULL UNIQUE,
    code text NOT NULL UNIQUE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Scores table with detailed game stats
CREATE TABLE IF NOT EXISTS scores (
    id integer PRIMARY KEY AUTOINCREMENT,
    user_id integer NOT NULL,
    score integer NOT NULL,
    floor_reached integer NOT NULL,
    play_time_seconds integer NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Indexes for performance
CREATE INDEX idx_users_login ON users (code);

CREATE INDEX idx_scores_user_score ON scores (user_id, score DESC);

