use rusqlite::{params, Result};
use crate::db::Database;
use crate::models::tag::{Tag, TagCreate};

pub fn list(db: &Database) -> Result<Vec<Tag>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, description, ignore_auto_tag, favorite,
                created_at, updated_at
         FROM tags ORDER BY name",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            ignore_auto_tag: row.get::<_, i64>(3)? != 0,
            favorite: row.get::<_, i64>(4)? != 0,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    rows.collect()
}

pub fn find(db: &Database, id: i64) -> Result<Option<Tag>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, description, ignore_auto_tag, favorite,
                created_at, updated_at
         FROM tags WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            ignore_auto_tag: row.get::<_, i64>(3)? != 0,
            favorite: row.get::<_, i64>(4)? != 0,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn create(db: &Database, input: &TagCreate) -> Result<Tag> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO tags (name, description, ignore_auto_tag, favorite)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            input.name,
            input.description,
            input.ignore_auto_tag.unwrap_or(false) as i64,
            input.favorite.unwrap_or(false) as i64,
        ],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    find(db, id).map(|opt| opt.unwrap())
}

pub fn destroy(db: &Database, id: i64) -> Result<bool> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM tags WHERE id = ?1", [id])?;
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
    fn test_tag_crud() {
        let db = test_db();
        assert!(list(&db).unwrap().is_empty());

        let created = create(&db, &TagCreate {
            name: "Test Tag".into(),
            description: Some("A test".into()),
            ignore_auto_tag: Some(false),
            favorite: Some(true),
        }).unwrap();
        assert_eq!(created.name, "Test Tag");
        assert!(created.favorite);

        let found = find(&db, created.id).unwrap().unwrap();
        assert_eq!(found.name, "Test Tag");

        let tags = list(&db).unwrap();
        assert_eq!(tags.len(), 1);

        assert!(destroy(&db, created.id).unwrap());
        assert!(find(&db, created.id).unwrap().is_none());
    }
}
