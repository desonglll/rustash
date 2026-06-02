CREATE TABLE IF NOT EXISTS images (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    code TEXT,
    details TEXT,
    photographer TEXT,
    url TEXT,
    date TEXT,
    rating INTEGER,
    organized INTEGER NOT NULL DEFAULT 0,
    studio_id INTEGER REFERENCES studios(id) ON DELETE SET NULL,
    o_counter INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX idx_images_studio ON images(studio_id);

CREATE TABLE IF NOT EXISTS images_files (
    image_id INTEGER NOT NULL REFERENCES images(id) ON DELETE CASCADE,
    file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    is_primary INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (image_id, file_id)
);

CREATE TABLE IF NOT EXISTS images_tags (
    image_id INTEGER NOT NULL REFERENCES images(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (image_id, tag_id)
);

CREATE TABLE IF NOT EXISTS images_performers (
    image_id INTEGER NOT NULL REFERENCES images(id) ON DELETE CASCADE,
    performer_id INTEGER NOT NULL REFERENCES performers(id) ON DELETE CASCADE,
    PRIMARY KEY (image_id, performer_id)
);

CREATE TABLE IF NOT EXISTS galleries (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    details TEXT,
    code TEXT,
    photographer TEXT,
    url TEXT,
    date TEXT,
    rating INTEGER,
    organized INTEGER NOT NULL DEFAULT 0,
    studio_id INTEGER REFERENCES studios(id) ON DELETE SET NULL,
    folder_id INTEGER REFERENCES folders(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE UNIQUE INDEX idx_galleries_folder ON galleries(folder_id);
CREATE INDEX idx_galleries_studio ON galleries(studio_id);

CREATE TABLE IF NOT EXISTS galleries_files (
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    file_id INTEGER NOT NULL REFERENCES files(id) ON DELETE CASCADE,
    is_primary INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (gallery_id, file_id)
);

CREATE TABLE IF NOT EXISTS gallery_images (
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    image_id INTEGER NOT NULL REFERENCES images(id) ON DELETE CASCADE,
    PRIMARY KEY (gallery_id, image_id)
);

CREATE TABLE IF NOT EXISTS gallery_chapters (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    image_index INTEGER NOT NULL,
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS galleries_tags (
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (gallery_id, tag_id)
);

CREATE TABLE IF NOT EXISTS galleries_performers (
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    performer_id INTEGER NOT NULL REFERENCES performers(id) ON DELETE CASCADE,
    PRIMARY KEY (gallery_id, performer_id)
);

CREATE TABLE IF NOT EXISTS galleries_scenes (
    gallery_id INTEGER NOT NULL REFERENCES galleries(id) ON DELETE CASCADE,
    scene_id INTEGER NOT NULL REFERENCES scenes(id) ON DELETE CASCADE,
    PRIMARY KEY (gallery_id, scene_id)
);
