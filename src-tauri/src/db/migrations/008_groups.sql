CREATE TABLE IF NOT EXISTS groups (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    aliases TEXT,
    duration INTEGER,
    date TEXT,
    rating INTEGER,
    studio_id INTEGER REFERENCES studios(id) ON DELETE SET NULL,
    director TEXT,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_groups_studio ON groups(studio_id);

CREATE TABLE IF NOT EXISTS scenes_groups (
    scene_id INTEGER NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    PRIMARY KEY (scene_id, group_id)
);
