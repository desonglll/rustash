use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

const MIGRATIONS: &[&str] = &[
    // v1: config table
    "CREATE TABLE IF NOT EXISTS config (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    )",
    // v2: tags (referenced by many other tables)
    "CREATE TABLE IF NOT EXISTS tags (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        description TEXT,
        ignore_auto_tag INTEGER NOT NULL DEFAULT 0,
        favorite INTEGER NOT NULL DEFAULT 0,
        created_at TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    )",
    // v3: studios (self-referencing, referenced by scenes/galleries/images/groups)
    "CREATE TABLE IF NOT EXISTS studios (
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
    CREATE INDEX idx_studios_parent ON studios(parent_id)",
    // v4: filesystem tables
    "CREATE TABLE IF NOT EXISTS folders (
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
    )",
    // v5: performers
    "CREATE TABLE IF NOT EXISTS performers (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        disambiguation TEXT,
        gender TEXT,
        url TEXT,
        birthdate TEXT,
        ethnicity TEXT,
        country TEXT,
        eye_color TEXT,
        height INTEGER,
        measurements TEXT,
        fake_tits TEXT,
        tattoos TEXT,
        piercings TEXT,
        favorite INTEGER NOT NULL DEFAULT 0,
        career_length TEXT,
        details TEXT,
        death_date TEXT,
        hair_color TEXT,
        weight INTEGER,
        rating INTEGER,
        ignore_auto_tag INTEGER NOT NULL DEFAULT 0,
        created_at TEXT NOT NULL DEFAULT (datetime('now')),
        updated_at TEXT NOT NULL DEFAULT (datetime('now'))
    );
    CREATE UNIQUE INDEX idx_performers_name ON performers(name)",
    // v6: scenes + join tables
    "CREATE TABLE IF NOT EXISTS scenes (
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
    )",
    // v7: images + galleries + join tables
    "CREATE TABLE IF NOT EXISTS images (
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
    )",
    // v8: groups + join table
    "CREATE TABLE IF NOT EXISTS groups (
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
    )",
    // v9: saved_filters + performers_tags
    "CREATE TABLE IF NOT EXISTS saved_filters (
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
    )",
];

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &PathBuf) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )?;

        let current_version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        for (i, sql) in MIGRATIONS.iter().enumerate() {
            let version = (i + 1) as i64;
            if version > current_version {
                conn.execute_batch(sql)?;
                conn.execute(
                    "INSERT INTO schema_migrations (version) VALUES (?1)",
                    [version],
                )?;
            }
        }

        Ok(())
    }
}

pub fn default_db_path() -> PathBuf {
    let dirs = directories::ProjectDirs::from("com", "shinoda", "rustash")
        .expect("Could not determine app data directory");
    let data_dir = dirs.data_dir();
    std::fs::create_dir_all(data_dir).ok();
    data_dir.join("rustash.db")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory().unwrap();
        let conn = db.conn.lock().unwrap();
        let version: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, 9);
    }

    #[test]
    fn test_config_table_exists() {
        let db = Database::open_in_memory().unwrap();
        let conn = db.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO config (key, value) VALUES ('test_key', 'test_value')",
            [],
        )
        .unwrap();
        let value: String = conn
            .query_row(
                "SELECT value FROM config WHERE key = 'test_key'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_all_core_tables_exist() {
        let db = Database::open_in_memory().unwrap();
        let conn = db.conn.lock().unwrap();
        let tables = [
            "config", "tags", "studios", "folders", "files",
            "video_files", "image_file_data", "performers", "scenes",
            "scenes_files", "scene_markers", "scenes_performers",
            "scenes_tags", "images", "images_files", "images_tags",
            "images_performers", "galleries", "galleries_files",
            "gallery_images", "gallery_chapters", "galleries_tags",
            "galleries_performers", "galleries_scenes", "groups",
            "scenes_groups", "saved_filters", "performers_tags",
        ];
        for table in &tables {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "Table {} should exist", table);
        }
    }
}
