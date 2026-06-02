use rusqlite::{params, Result};
use crate::db::Database;
use crate::models::scene::{Scene, SceneCreate, SceneUpdate};

pub fn list(db: &Database, page: i64, per_page: i64) -> Result<Vec<Scene>> {
    let conn = db.conn.lock().unwrap();
    let offset = (page - 1).max(0) * per_page;
    let mut stmt = conn.prepare(
        "SELECT id, title, details, code, director, url, date, rating,
                organized, studio_id, resume_time, play_duration,
                created_at, updated_at
         FROM scenes ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt.query_map([per_page, offset], |row| {
        Ok(Scene {
            id: row.get(0)?,
            title: row.get(1)?,
            details: row.get(2)?,
            code: row.get(3)?,
            director: row.get(4)?,
            url: row.get(5)?,
            date: row.get(6)?,
            rating: row.get(7)?,
            organized: row.get::<_, i64>(8)? != 0,
            studio_id: row.get(9)?,
            resume_time: row.get(10)?,
            play_duration: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    })?;
    rows.collect()
}

pub fn find(db: &Database, id: i64) -> Result<Option<Scene>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, title, details, code, director, url, date, rating,
                organized, studio_id, resume_time, play_duration,
                created_at, updated_at
         FROM scenes WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Scene {
            id: row.get(0)?,
            title: row.get(1)?,
            details: row.get(2)?,
            code: row.get(3)?,
            director: row.get(4)?,
            url: row.get(5)?,
            date: row.get(6)?,
            rating: row.get(7)?,
            organized: row.get::<_, i64>(8)? != 0,
            studio_id: row.get(9)?,
            resume_time: row.get(10)?,
            play_duration: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn count(db: &Database) -> Result<i64> {
    let conn = db.conn.lock().unwrap();
    conn.query_row("SELECT COUNT(*) FROM scenes", [], |row| row.get(0))
}

pub fn create(db: &Database, input: &SceneCreate) -> Result<Scene> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO scenes (title, details, code, director, url, date, rating, studio_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.title,
            input.details,
            input.code,
            input.director,
            input.url,
            input.date,
            input.rating,
            input.studio_id,
        ],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    find(db, id).map(|opt| opt.unwrap())
}

pub fn update(db: &Database, input: &SceneUpdate) -> Result<Option<Scene>> {
    let conn = db.conn.lock().unwrap();
    let existing = {
        let mut stmt = conn.prepare(
            "SELECT organized, resume_time FROM scenes WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map([input.id], |row| {
            Ok((row.get::<_, i64>(0)? != 0, row.get::<_, f64>(1)?))
        })?;
        rows.next().transpose()
    };
    let (organized, resume_time) = match existing? {
        Some(v) => v,
        None => return Ok(None),
    };
    conn.execute(
        "UPDATE scenes SET
            title = COALESCE(?1, title),
            details = COALESCE(?2, details),
            code = COALESCE(?3, code),
            director = COALESCE(?4, director),
            url = COALESCE(?5, url),
            date = COALESCE(?6, date),
            rating = COALESCE(?7, rating),
            organized = COALESCE(?8, ?9),
            studio_id = COALESCE(?10, studio_id),
            resume_time = COALESCE(?11, ?12),
            updated_at = datetime('now')
         WHERE id = ?13",
        params![
            input.title,
            input.details,
            input.code,
            input.director,
            input.url,
            input.date,
            input.rating,
            input.organized.map(|b| b as i64),
            organized as i64,
            input.studio_id,
            input.resume_time,
            resume_time,
            input.id,
        ],
    )?;
    drop(conn);
    find(db, input.id)
}

pub fn destroy(db: &Database, id: i64) -> Result<bool> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM scenes WHERE id = ?1", [id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn test_db() -> Database {
        Database::open_in_memory().unwrap()
    }

    #[test]
    fn test_scene_crud() {
        let db = test_db();
        assert_eq!(count(&db).unwrap(), 0);

        let created = create(&db, &SceneCreate {
            title: Some("Test Scene".into()),
            details: None,
            code: None,
            director: None,
            url: None,
            date: Some("2024-01-01".into()),
            rating: Some(5),
            studio_id: None,
        }).unwrap();
        assert_eq!(created.title, Some("Test Scene".into()));
        assert_eq!(created.id, 1);

        let found = find(&db, 1).unwrap().unwrap();
        assert_eq!(found.title, Some("Test Scene".into()));

        let updated = update(&db, &SceneUpdate {
            id: 1,
            title: Some("Updated Scene".into()),
            details: Some("New details".into()),
            code: None,
            director: None,
            url: None,
            date: None,
            rating: Some(4),
            organized: Some(true),
            studio_id: None,
            resume_time: None,
        }).unwrap().unwrap();
        assert_eq!(updated.title, Some("Updated Scene".into()));
        assert!(updated.organized);

        assert_eq!(count(&db).unwrap(), 1);
        let lists = list(&db, 1, 10).unwrap();
        assert_eq!(lists.len(), 1);

        assert!(destroy(&db, 1).unwrap());
        assert!(find(&db, 1).unwrap().is_none());
        assert_eq!(count(&db).unwrap(), 0);
    }

    #[test]
    fn test_scene_pagination() {
        let db = test_db();
        for i in 0..15 {
            create(&db, &SceneCreate {
                title: Some(format!("Scene {}", i)),
                details: None, code: None, director: None,
                url: None, date: None, rating: None, studio_id: None,
            }).unwrap();
        }
        let page1 = list(&db, 1, 10).unwrap();
        assert_eq!(page1.len(), 10);
        let page2 = list(&db, 2, 10).unwrap();
        assert_eq!(page2.len(), 5);
    }
}
