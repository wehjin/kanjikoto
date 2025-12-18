use crate::core::data::{Lesson, Phrase, User};
use rusqlite::params;
use std::path::Path;
use thiserror::Error;

pub mod lesson;
pub mod misc;

mod insert_lesson;
pub use insert_lesson::InsertLesson;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

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
    conn
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
    use crate::core::backend::insert_lesson::UpdateLessonTimes;
    use crate::core::backend::lesson::{QueryLessonStatus, QueryPracticeCards};
    use crate::core::backend::{connect, get_users, read_phrases, read_user_lesson, InsertLesson};
    use crate::core::data::lesson_status::LessonStatus;
    use crate::core::data::NewPhrase;

    pub fn today_at_3am(db: &rusqlite::Connection) -> Result<f64, rusqlite::Error> {
        db.query_row(
            "SELECT julianday('now','localtime', 'start of day', '+3 hours')",
            [],
            |row| Ok(row.get(0)?),
        )
    }

    #[test]
    fn it_works() {
        let mut conn = connect(None);
        let admin = {
            let users = get_users(&conn);
            assert_eq!(users.len(), 1);
            users.into_iter().next().unwrap()
        };

        let today_at_3am = today_at_3am(&conn).unwrap();
        InsertLesson {
            title: "Aggrieved Ch1".to_string(),
            owner: admin.id.clone(),
            phrases: vec![
                NewPhrase {
                    prompt: "嫌".to_string(),
                    reading: "いや".to_string(),
                    translation: "unpleasant".to_string(),
                    content_changed_at: Some(today_at_3am - 0.1),
                },
                NewPhrase {
                    prompt: "必要".to_string(),
                    reading: "ひつよう".to_string(),
                    translation: "necessary".to_string(),
                    content_changed_at: Some(today_at_3am - 0.1),
                },
            ],
        }
        .apply(&mut conn)
        .expect("Failed to insert lesson");

        let lesson = read_user_lesson(&admin.id, &conn)
            .expect("Failed to fetch lesson")
            .unwrap();
        assert_eq!(lesson.title, "Aggrieved Ch1");
        let phrases = read_phrases(lesson.lesson_id, &conn).expect("Failed to fetch phrases");
        let phrase_ids = phrases.iter().map(|p| p.phrase_id).collect::<Vec<_>>();
        assert_eq!(phrase_ids.len(), 2);
        let mut now = today_at_3am + 0.11;
        {
            let lesson_status = QueryLessonStatus {
                lesson_id: lesson.lesson_id,
                now,
            }
            .apply(&conn)
            .expect("Failed to fetch lesson status");
            assert_eq!(
                lesson_status,
                LessonStatus {
                    ready: 2,
                    learned: 0
                }
            );
            let practice_cards = QueryPracticeCards {
                lesson_id: lesson.lesson_id,
                now,
            }
            .apply(&conn)
            .expect("Failed to fetch practice cards");
            assert_eq!(practice_cards.len(), 2);
        }
        now += 0.01;
        {
            UpdateLessonTimes {
                phrase_ids: vec![phrase_ids[0]],
                now,
            }
            .apply(&mut conn)
            .expect("Failed to update lesson time");
            assert_eq!(
                QueryLessonStatus {
                    lesson_id: lesson.lesson_id,
                    now,
                }
                .apply(&conn)
                .expect("Failed to fetch lesson status"),
                LessonStatus {
                    ready: 1,
                    learned: 1
                }
            );
        }
        now += 0.01;
        {
            UpdateLessonTimes {
                phrase_ids: vec![phrase_ids[1]],
                now,
            }
            .apply(&mut conn)
            .expect("Failed to update lesson time");
            assert_eq!(
                QueryLessonStatus {
                    lesson_id: lesson.lesson_id,
                    now,
                }
                .apply(&conn)
                .expect("Failed to fetch lesson status"),
                LessonStatus {
                    ready: 0,
                    learned: 2
                }
            );
        }
        now += 1.0;
        {
            assert_eq!(
                QueryLessonStatus {
                    lesson_id: lesson.lesson_id,
                    now,
                }
                .apply(&conn)
                .expect("Failed to fetch lesson status"),
                LessonStatus {
                    ready: 2,
                    learned: 0,
                }
            );
        }
    }
}
