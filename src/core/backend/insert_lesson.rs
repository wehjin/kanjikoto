use crate::core::backend::StorageError;
use crate::core::data::NewPhrase;
use rusqlite::params;

pub struct UpdateLessonTimes {
    pub phrase_ids: Vec<i64>,
    pub now: f64,
}
impl UpdateLessonTimes {
    pub fn apply(self, conn: &mut rusqlite::Connection) -> Result<(), StorageError> {
        let tx = conn.transaction()?;
        {
            const SQL: &str = "UPDATE phrases SET learned_at = ?1 WHERE id = ?2";
            let mut stmt = tx.prepare(SQL)?;
            for id in self.phrase_ids {
                stmt.execute(params![self.now, id])?;
            }
        }
        tx.commit()?;
        Ok(())
    }
}

pub struct InsertLesson {
    pub title: String,
    pub owner: String,
    pub phrases: Vec<NewPhrase>,
}

impl InsertLesson {
    pub fn apply(self, conn: &mut rusqlite::Connection) -> Result<i64, StorageError> {
        let tx = conn.transaction()?;
        let lesson_id: i64 = {
            const SQL: &str =
                "INSERT INTO lessons (title, creator_id) VALUES (?1, ?2) RETURNING id";
            let mut stmt = tx.prepare(SQL)?;
            stmt.query_row(params![self.title, self.owner], |row| row.get(0))?
        };
        {
            const SQL_4: &str =
                "INSERT INTO phrases (lesson_id, prompt, reading, translation) VALUES (?1, ?2, ?3, ?4)";
            const SQL_5: &str =
                "INSERT INTO phrases (lesson_id, prompt, reading, translation, content_changed_at) VALUES (?1, ?2, ?3, ?4, ?5)";
            let mut stmt_4 = tx.prepare(SQL_4)?;
            let mut stmt_5 = tx.prepare(SQL_5)?;
            for phrase in self.phrases {
                if let Some(content_changed_at) = phrase.content_changed_at {
                    stmt_5.execute(params![
                        lesson_id,
                        phrase.prompt,
                        phrase.reading,
                        phrase.translation,
                        content_changed_at
                    ])?;
                } else {
                    stmt_4.execute(params![
                        lesson_id,
                        phrase.prompt,
                        phrase.reading,
                        phrase.translation
                    ])?;
                }
            }
        }
        tx.commit()?;
        Ok(lesson_id)
    }
}
