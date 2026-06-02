CREATE TABLE IF NOT EXISTS folders (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    parent_folder_id INTEGER REFERENCES folders(id) ON DELETE SET NULL,
    mod_time TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_folders_parent ON folders(parent_folder_id);

CREATE TABLE IF NOT EXISTS files (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    basename TEXT NOT NULL CHECK (basename != ''),
    parent_folder_id INTEGER NOT NULL REFERENCES folders(id),
    size INTEGER NOT NULL,
    mod_time TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE UNIQUE INDEX idx_files_folder_basename ON files(parent_folder_id, basename);

CREATE TABLE IF NOT EXISTS video_files (
    file_id INTEGER NOT NULL PRIMARY KEY REFERENCES files(id) ON DELETE CASCADE,
    duration REAL NOT NULL,
    video_codec TEXT NOT NULL,
    format TEXT NOT NULL,
    audio_codec TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    frame_rate REAL NOT NULL,
    bit_rate INTEGER NOT NULL,
    interactive INTEGER NOT NULL DEFAULT 0,
    interactive_speed INTEGER
);

CREATE TABLE IF NOT EXISTS image_file_data (
    file_id INTEGER NOT NULL PRIMARY KEY REFERENCES files(id) ON DELETE CASCADE,
    format TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL
);
