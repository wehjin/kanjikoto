use crate::core::backend::StorageError;
use rusqlite::params;
use crate::core::data::LessonStatus;

pub struct QueryLessonStatus {
    pub lesson_id: i64,
    pub now: f64,
}
impl QueryLessonStatus {
    pub fn apply(self, db: &rusqlite::Connection) -> Result<LessonStatus, StorageError> {
        let lesson_id = self.lesson_id;
        const SQL: &str = r#"
SELECT
	COUNT(CASE
		WHEN learned_at IS NULL THEN 1
		WHEN learned_at < julianday(?1, 'start of day', '+3 hours') THEN 1
		WHEN content_changed_at > learned_at THEN 1
		ELSE NULL
	END) as count_before_3am,
	COUNT(*) as count_all
FROM
	 phrases
WHERE
	lesson_id = ?2;
"#;
        let status = db.query_row(SQL, params![self.now, lesson_id], |row| {
            let ready_count: i64 = row.get(0)?;
            let total_count: i64 = row.get(1)?;
            Ok(LessonStatus {
                ready: ready_count as usize,
                learned: (total_count - ready_count) as usize,
            })
        })?;
        Ok(status)
    }
}
