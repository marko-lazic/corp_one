-- Create players table
CREATE TABLE IF NOT EXISTS players (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    character_name TEXT NOT NULL,
    current_health INTEGER NOT NULL DEFAULT 100,
    credits INTEGER NOT NULL DEFAULT 1000,
    faction TEXT NOT NULL,
    rank TEXT NOT NULL DEFAULT 'R0',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create indices for better performance
CREATE INDEX IF NOT EXISTS idx_players_username ON players(username);
CREATE INDEX IF NOT EXISTS idx_players_faction ON players(faction);
CREATE INDEX IF NOT EXISTS idx_players_rank ON players(rank);

-- Enable WAL mode for better performance
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = 1000;
PRAGMA foreign_keys = true;
PRAGMA temp_store = memory;