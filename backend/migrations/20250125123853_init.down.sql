-- Add down migration script here
-- Drop indexes first
DROP INDEX IF EXISTS idx_scores_user_score;

DROP INDEX IF EXISTS idx_users_login;

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS scores;

DROP TABLE IF EXISTS users;

