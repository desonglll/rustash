CREATE TABLE IF NOT EXISTS scenes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    details TEXT,
    code TEXT,
    director TEXT,
    url TEXT,
    date TEXT,
    rating INTEGER,
    organized INTEGER NOT NULL DEFAULT 0,
    studio_id INTEGER REFERENCES studios(id) ON DELETE SET NULL,
    resume_time REAL NOT NULL DEFAULT 0,
    play_duration REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_scenes_studio ON scenes(studio_id);

CREATE TABLE IF NOT EXISTS scenes_files (
    scene_id INTEGER NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    is_primary INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (scene_id, file_id)
);

CREATE TABLE IF NOT EXISTS scene_markers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    seconds REAL NOT NULL,
    end_seconds REAL,
    primary_tag_id INTEGER NOT NULL REFERENCES tags(id),
    scene_id INTEGER NOT NULL REFERENCES scenes(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_scene_markers_scene ON scene_markers(scene_id);
CREATE INDEX idx_scene_markers_tag ON scene_markers(primary_tag_id);

CREATE TABLE IF NOT EXISTS scenes_performers (
    scene_id INTEGER NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    performer_id INTEGER NOT NULL REFERENCES performers(id) ON DELETE CASCADE,
    PRIMARY KEY (scene_id, performer_id)
);

CREATE TABLE IF NOT EXISTS scenes_tags (
    scene_id INTEGER NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (scene_id, tag_id)
);
