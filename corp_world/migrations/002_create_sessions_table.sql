-- Create player sessions table
CREATE TABLE IF NOT EXISTS player_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    token TEXT,
    colony TEXT,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_heartbeat DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    FOREIGN KEY (player_id) REFERENCES players (id) ON DELETE CASCADE
);

-- Create indices for better performance
CREATE INDEX IF NOT EXISTS idx_sessions_player_id ON player_sessions(player_id);
CREATE INDEX IF NOT EXISTS idx_sessions_username ON player_sessions(username);
CREATE INDEX IF NOT EXISTS idx_sessions_is_active ON player_sessions(is_active);
CREATE INDEX IF NOT EXISTS idx_sessions_last_heartbeat ON player_sessions(last_heartbeat);
CREATE INDEX IF NOT EXISTS idx_sessions_token ON player_sessions(token);