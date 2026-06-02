CREATE TABLE IF NOT EXISTS saved_filters (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    mode TEXT NOT NULL,
    find_filter TEXT,
    object_filter TEXT,
    ui_options TEXT
);
CREATE UNIQUE INDEX idx_saved_filters_mode_name ON saved_filters(mode, name);

CREATE TABLE IF NOT EXISTS performers_tags (
    performer_id INTEGER NOT NULL REFERENCES performers(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (performer_id, tag_id)
);
