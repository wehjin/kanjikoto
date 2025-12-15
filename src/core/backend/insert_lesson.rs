use crate::core::backend::StorageError;
use crate::core::data::NewPhrase;
use rusqlite::params;

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
            const SQL: &str =
                "INSERT INTO phrases (lesson_id, prompt, reading, translation) VALUES (?1, ?2, ?3, ?4)";
            let mut stmt = tx.prepare(SQL)?;
            for phrase in self.phrases {
                stmt.execute(params![
                    lesson_id,
                    phrase.prompt,
                    phrase.reading,
                    phrase.translation
                ])?;
            }
        }
        tx.commit()?;
        Ok(lesson_id)
    }
}
