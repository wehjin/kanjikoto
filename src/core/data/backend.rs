use crate::core::data::{Lesson, User};
use rusqlite::params;
use std::path::Path;
use thiserror::Error;

pub fn connect(filename: Option<&'static str>) -> rusqlite::Connection {
    let (conn, existed) = if let Some(filename) = filename {
        let db_existed = Path::new(filename).exists();
        let conn = rusqlite::Connection::open(filename).expect("Failed to open database");
        (conn, db_existed)
    } else {
        let conn =
            rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
        (conn, false)
    };
    conn.pragma_update(None, "foreign_keys", "ON")
        .expect("Failed to enable foreign keys");

    let admin = User {
        id: "admin".to_string(),
        is_admin: true,
    };
    if !existed {
        let up = include_str!("up.sql");
        conn.execute_batch(up)
            .expect("Failed to run database upgrade");
        conn.execute(
            "INSERT INTO users (id, is_admin) VALUES (?1, ?2)",
            (&admin.id, &admin.is_admin),
        )
        .expect("Failed to create admin user");
    }
    let lessons = read_user_lesson(&admin.id, &conn).expect("Failed to fetch lessons");
    if lessons.is_none() {
        insert_user_lesson(&admin.id, "Aggrieved Ch1", &conn).expect("Failed to create lesson");
    }
    conn
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

fn insert_user_lesson(
    creator: &str,
    title: &str,
    conn: &rusqlite::Connection,
) -> Result<i64, StorageError> {
    const SQL: &str = "INSERT INTO lessons (title, creator_id) VALUES (?1, ?2) RETURNING id";
    let mut stmt = conn.prepare(SQL)?;
    let lesson_id: i64 = stmt.query_row(params![title, creator], |row| row.get(0))?;
    Ok(lesson_id)
}

pub fn read_user_lesson(
    user: &str,
    conn: &rusqlite::Connection,
) -> Result<Option<Lesson>, StorageError> {
    const SQL: &str = "SELECT id, title, creator_id FROM lessons WHERE creator_id = ?1";
    let mut stmt = conn.prepare(SQL)?;
    let lesson_iter = stmt.query_map(params![user], |row| {
        let lesson_id = row.get(0)?;
        let title = row.get(1)?;
        let creator_id = row.get(2)?;
        Ok(Lesson {
            lesson_id,
            title,
            creator_id,
        })
    })?;
    let lessons = lesson_iter
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .next();
    Ok(lessons)
}

pub fn get_users(conn: &rusqlite::Connection) -> Vec<User> {
    let mut stmt = conn.prepare("SELECT id, is_admin FROM users").unwrap();
    let user_iter = stmt
        .query_map([], |row| {
            let id = row.get(0).unwrap();
            let is_admin = row.get(1).unwrap();
            Ok(User { id, is_admin })
        })
        .expect("Failed to fetch users");
    user_iter.collect::<Result<Vec<_>, _>>().unwrap()
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() -> Result<(), anyhow::Error> {
        let conn = super::connect(None);
        let users = super::get_users(&conn);
        assert_eq!(users.len(), 1);

        let admin = users.into_iter().next().unwrap();
        let lesson = super::read_user_lesson(&admin.id, &conn)?.unwrap();
        assert_eq!(lesson.title, "Aggrieved Ch1");
        Ok(())
    }
}
