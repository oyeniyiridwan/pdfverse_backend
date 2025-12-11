-- ========================
-- USERS TABLE
-- ========================

-- Create users table if it doesn't exist (for new deployments)
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(64) DEFAULT NULL,
    last_name VARCHAR(64) DEFAULT NULL,
    email VARCHAR(128) DEFAULT NULL UNIQUE,
    password VARCHAR(64) DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    token TEXT DEFAULT NULL
);

-- ========================
-- USERS TABLE MIGRATION (for existing DBs)
-- ========================

-- 1) Remove old username column
-- 2) Add new columns if they don't exist
ALTER TABLE users
ADD COLUMN IF NOT EXISTS first_name VARCHAR(64) DEFAULT NULL,
ADD COLUMN IF NOT EXISTS last_name VARCHAR(64) DEFAULT NULL,
ADD COLUMN IF NOT EXISTS email VARCHAR(128) DEFAULT NULL;





-- Drop username if exists
ALTER TABLE users DROP COLUMN IF EXISTS username;

-- Update columns safely
ALTER TABLE users ALTER COLUMN first_name DROP NOT NULL;
ALTER TABLE users ALTER COLUMN first_name SET DEFAULT NULL;

ALTER TABLE users ALTER COLUMN last_name DROP NOT NULL;
ALTER TABLE users ALTER COLUMN last_name SET DEFAULT NULL;

ALTER TABLE users ALTER COLUMN email DROP NOT NULL;
ALTER TABLE users ALTER COLUMN email SET DEFAULT NULL;

ALTER TABLE users ALTER COLUMN password DROP NOT NULL;
ALTER TABLE users ALTER COLUMN password SET DEFAULT NULL;





-- 4) Add index for faster email lookup
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);


-- ========================
-- TASKS TABLE
-- ========================

CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    priority VARCHAR(4) DEFAULT NULL,
    title VARCHAR(255) NOT NULL,
    completed_at TIMESTAMPTZ DEFAULT NULL,
    description TEXT DEFAULT NULL,
    deleted_at TIMESTAMPTZ DEFAULT NULL,
    user_id INTEGER DEFAULT NULL,
    is_default BOOLEAN DEFAULT FALSE,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE SET NULL
);

-- Index for fetching tasks by user_id efficiently
CREATE INDEX IF NOT EXISTS idx_tasks_user_id ON tasks(user_id);


-- ========================
-- PROVIDERS TABLE
-- ========================

CREATE TABLE IF NOT EXISTS providers (
    id SERIAL PRIMARY KEY,
    provider_name VARCHAR(32) NOT NULL,
    provider_user_id VARCHAR(128) NOT NULL,
    access_token TEXT DEFAULT NULL,
    refresh_token TEXT DEFAULT NULL,
    expires_at TIMESTAMPTZ DEFAULT NULL,
    user_id INTEGER NOT NULL,
    email VARCHAR(128) DEFAULT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_provider_user FOREIGN KEY (user_id) REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT unique_provider_user UNIQUE (provider_name, provider_user_id)
);
ALTER TABLE providers
ADD COLUMN IF NOT EXISTS email VARCHAR(128) DEFAULT NULL;


-- Indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_providers_user_id ON providers(user_id);
CREATE INDEX IF NOT EXISTS idx_providers_provider_name ON providers(provider_name);
CREATE INDEX IF NOT EXISTS idx_providers_provider_and_userid
    ON providers(provider_name, provider_user_id);
