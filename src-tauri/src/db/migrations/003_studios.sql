CREATE TABLE IF NOT EXISTS studios (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    url TEXT,
    parent_id INTEGER REFERENCES studios(id) ON DELETE SET NULL CHECK (id != parent_id),
    details TEXT,
    rating INTEGER,
    ignore_auto_tag INTEGER NOT NULL DEFAULT 0,
    favorite INTEGER NOT NULL DEFAULT 0,
    organized INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_studios_parent ON studios(parent_id);
