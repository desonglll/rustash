use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::Mutex;

/// Migrations loaded from individual SQL files at compile time via `include_str!`.
/// To add a new migration, create a new numbered SQL file in migrations/ and
/// add a corresponding `include_str!` entry below.
const MIGRATIONS: &[&str] = &[
    include_str!("migrations/001_config.sql"),
    include_str!("migrations/002_tags.sql"),
    include_str!("migrations/003_studios.sql"),
    include_str!("migrations/004_filesystem.sql"),
    include_str!("migrations/005_performers.sql"),
    include_str!("migrations/006_scenes.sql"),
    include_str!("migrations/007_images_galleries.sql"),
    include_str!("migrations/008_groups.sql"),
    include_str!("migrations/009_saved_filters.sql"),
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
