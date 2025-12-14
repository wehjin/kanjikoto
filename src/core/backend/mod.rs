use crate::core::data::{Lesson, NewPhrase, Phrase, User};
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

pub fn insert_phrases(
    phrases: Vec<NewPhrase>,
    conn: &mut rusqlite::Connection,
) -> Result<(), StorageError> {
    const SQL: &str =
        "INSERT INTO phrases (lesson_id, prompt, reading, translation) VALUES (?1, ?2, ?3, ?4)";
    let tx = conn.transaction()?;
    {
        let mut stmt = tx.prepare(SQL)?;
        for phrase in phrases {
            stmt.execute(params![
                phrase.lesson_id,
                phrase.prompt,
                phrase.reading,
                phrase.translation
            ])?;
        }
        stmt.finalize()?;
    }
    tx.commit()?;
    Ok(())
}
pub fn read_phrases(
    lesson_id: i64,
    conn: &rusqlite::Connection,
) -> Result<Vec<Phrase>, StorageError> {
    const SQL: &str = "SELECT id, prompt, reading, translation FROM phrases WHERE lesson_id = ?1";
    let mut stmt = conn.prepare(SQL)?;
    let phrases = stmt
        .query_map(params![lesson_id], |row| {
            Ok(Phrase {
                phrase_id: row.get(0)?,
                lesson_id,
                prompt: row.get(1)?,
                reading: row.get(2)?,
                translation: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(phrases)
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
    let lesson = stmt
        .query_map(params![user], |row| {
            let lesson_id = row.get(0)?;
            let title = row.get(1)?;
            let creator_id = row.get(2)?;
            Ok(Lesson {
                lesson_id,
                title,
                creator_id,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .next();
    Ok(lesson)
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
    use crate::core::data::NewPhrase;

    #[test]
    fn it_works() {
        let mut conn = super::connect(None);
        let users = super::get_users(&conn);
        assert_eq!(users.len(), 1);

        let admin = users.into_iter().next().unwrap();
        let lesson = super::read_user_lesson(&admin.id, &conn)
            .expect("Failed to fetch lesson")
            .unwrap();
        assert_eq!(lesson.title, "Aggrieved Ch1");

        let new_phrases = vec![NewPhrase {
            lesson_id: lesson.lesson_id,
            prompt: "嫌".to_string(),
            reading: "いや".to_string(),
            translation: "unpleasant".to_string(),
        }];
        super::insert_phrases(new_phrases, &mut conn).expect("Failed to insert phrases");
        let phrases =
            super::read_phrases(lesson.lesson_id, &conn).expect("Failed to fetch phrases");
        assert_eq!(phrases.len(), 1);
    }
}
