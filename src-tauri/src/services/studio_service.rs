use rusqlite::{params, Result};
use crate::db::Database;
use crate::models::studio::{Studio, StudioCreate};

pub fn list(db: &Database) -> Result<Vec<Studio>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, url, parent_id, details, rating,
                ignore_auto_tag, favorite, organized,
                created_at, updated_at
         FROM studios ORDER BY name",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Studio {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            parent_id: row.get(3)?,
            details: row.get(4)?,
            rating: row.get(5)?,
            ignore_auto_tag: row.get::<_, i64>(6)? != 0,
            favorite: row.get::<_, i64>(7)? != 0,
            organized: row.get::<_, i64>(8)? != 0,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    rows.collect()
}

pub fn find(db: &Database, id: i64) -> Result<Option<Studio>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, url, parent_id, details, rating,
                ignore_auto_tag, favorite, organized,
                created_at, updated_at
         FROM studios WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Studio {
            id: row.get(0)?,
            name: row.get(1)?,
            url: row.get(2)?,
            parent_id: row.get(3)?,
            details: row.get(4)?,
            rating: row.get(5)?,
            ignore_auto_tag: row.get::<_, i64>(6)? != 0,
            favorite: row.get::<_, i64>(7)? != 0,
            organized: row.get::<_, i64>(8)? != 0,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn create(db: &Database, input: &StudioCreate) -> Result<Studio> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO studios (name, url, parent_id, details, rating)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![input.name, input.url, input.parent_id, input.details, input.rating],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    find(db, id).map(|opt| opt.unwrap())
}

pub fn destroy(db: &Database, id: i64) -> Result<bool> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM studios WHERE id = ?1", [id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn test_studio_crud() {
        let db = Database::open_in_memory().unwrap();
        let created = create(&db, &StudioCreate {
            name: "Test Studio".into(),
            url: Some("https://example.com".into()),
            parent_id: None,
            details: None,
            rating: None,
        }).unwrap();
        assert_eq!(created.name, "Test Studio");
        assert_eq!(created.url, Some("https://example.com".into()));

        let found = find(&db, created.id).unwrap().unwrap();
        assert_eq!(found.name, "Test Studio");

        let list = list(&db).unwrap();
        assert_eq!(list.len(), 1);

        assert!(destroy(&db, created.id).unwrap());
        assert!(find(&db, created.id).unwrap().is_none());
    }
}
