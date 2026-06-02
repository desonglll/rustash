use rusqlite::{params, Result};
use crate::db::Database;
use crate::models::performer::{Performer, PerformerCreate};

pub fn list(db: &Database, page: i64, per_page: i64) -> Result<Vec<Performer>> {
    let conn = db.conn.lock().unwrap();
    let offset = (page - 1).max(0) * per_page;
    let mut stmt = conn.prepare(
        "SELECT id, name, disambiguation, gender, url, birthdate,
                ethnicity, country, eye_color, height, measurements,
                fake_tits, tattoos, piercings, favorite, career_length,
                details, death_date, hair_color, weight, rating,
                ignore_auto_tag, created_at, updated_at
         FROM performers ORDER BY name LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt.query_map([per_page, offset], |row| {
        Ok(Performer {
            id: row.get(0)?,
            name: row.get(1)?,
            disambiguation: row.get(2)?,
            gender: row.get(3)?,
            url: row.get(4)?,
            birthdate: row.get(5)?,
            ethnicity: row.get(6)?,
            country: row.get(7)?,
            eye_color: row.get(8)?,
            height: row.get(9)?,
            measurements: row.get(10)?,
            fake_tits: row.get(11)?,
            tattoos: row.get(12)?,
            piercings: row.get(13)?,
            favorite: row.get::<_, i64>(14)? != 0,
            career_length: row.get(15)?,
            details: row.get(16)?,
            death_date: row.get(17)?,
            hair_color: row.get(18)?,
            weight: row.get(19)?,
            rating: row.get(20)?,
            ignore_auto_tag: row.get::<_, i64>(21)? != 0,
            created_at: row.get(22)?,
            updated_at: row.get(23)?,
        })
    })?;
    rows.collect()
}

pub fn count(db: &Database) -> Result<i64> {
    let conn = db.conn.lock().unwrap();
    conn.query_row("SELECT COUNT(*) FROM performers", [], |row| row.get(0))
}

pub fn find(db: &Database, id: i64) -> Result<Option<Performer>> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, disambiguation, gender, url, birthdate,
                ethnicity, country, eye_color, height, measurements,
                fake_tits, tattoos, piercings, favorite, career_length,
                details, death_date, hair_color, weight, rating,
                ignore_auto_tag, created_at, updated_at
         FROM performers WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map([id], |row| {
        Ok(Performer {
            id: row.get(0)?,
            name: row.get(1)?,
            disambiguation: row.get(2)?,
            gender: row.get(3)?,
            url: row.get(4)?,
            birthdate: row.get(5)?,
            ethnicity: row.get(6)?,
            country: row.get(7)?,
            eye_color: row.get(8)?,
            height: row.get(9)?,
            measurements: row.get(10)?,
            fake_tits: row.get(11)?,
            tattoos: row.get(12)?,
            piercings: row.get(13)?,
            favorite: row.get::<_, i64>(14)? != 0,
            career_length: row.get(15)?,
            details: row.get(16)?,
            death_date: row.get(17)?,
            hair_color: row.get(18)?,
            weight: row.get(19)?,
            rating: row.get(20)?,
            ignore_auto_tag: row.get::<_, i64>(21)? != 0,
            created_at: row.get(22)?,
            updated_at: row.get(23)?,
        })
    })?;
    match rows.next() {
        Some(row) => Ok(Some(row?)),
        None => Ok(None),
    }
}

pub fn create(db: &Database, input: &PerformerCreate) -> Result<Performer> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO performers (name, disambiguation, gender, url, birthdate,
                ethnicity, country, eye_color, height, measurements,
                fake_tits, tattoos, piercings, career_length, details,
                death_date, hair_color, weight)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![
            input.name, input.disambiguation, input.gender, input.url,
            input.birthdate, input.ethnicity, input.country, input.eye_color,
            input.height, input.measurements, input.fake_tits, input.tattoos,
            input.piercings, input.career_length, input.details, input.death_date,
            input.hair_color, input.weight,
        ],
    )?;
    let id = conn.last_insert_rowid();
    drop(conn);
    find(db, id).map(|opt| opt.unwrap())
}

pub fn destroy(db: &Database, id: i64) -> Result<bool> {
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute("DELETE FROM performers WHERE id = ?1", [id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    #[test]
    fn test_performer_crud() {
        let db = Database::open_in_memory().unwrap();
        let created = create(&db, &PerformerCreate {
            name: "Test Performer".into(),
            disambiguation: None,
            gender: Some("Female".into()),
            url: None,
            birthdate: None,
            ethnicity: None,
            country: Some("US".into()),
            eye_color: None,
            height: None,
            measurements: None,
            fake_tits: None,
            tattoos: None,
            piercings: None,
            career_length: None,
            details: None,
            death_date: None,
            hair_color: None,
            weight: None,
        }).unwrap();
        assert_eq!(created.name, "Test Performer");

        let found = find(&db, created.id).unwrap().unwrap();
        assert_eq!(found.gender, Some("Female".into()));

        assert_eq!(count(&db).unwrap(), 1);
        assert!(destroy(&db, created.id).unwrap());
        assert!(find(&db, created.id).unwrap().is_none());
    }
}
