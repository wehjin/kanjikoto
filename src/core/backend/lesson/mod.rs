use crate::core::backend::StorageError;
use crate::core::data::card::{Card, CardBack, CardFront, Goal};
use crate::core::data::lesson_status::{LessonStatus, SESSION_SIZE};
use rand::prelude::SliceRandom;
use rusqlite::{params, Connection};

pub struct QueryLessonStatus {
    pub lesson_id: i64,
    pub now: f64,
}

impl QueryLessonStatus {
    pub fn apply(self, db: &Connection) -> Result<LessonStatus, StorageError> {
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
                lesson_id,
                ready: ready_count as usize,
                learned: (total_count - ready_count) as usize,
            })
        })?;
        Ok(status)
    }
}

pub struct QueryPracticeCards {
    pub lesson_id: i64,
    pub now: f64,
}

impl QueryPracticeCards {
    pub fn apply(self, db: &Connection) -> Result<Vec<Card>, StorageError> {
        let Self { lesson_id, now } = self;
        let mut cards = select_ready(db, lesson_id, now)?;
        if cards.len() < SESSION_SIZE {
            let fill_count = SESSION_SIZE - cards.len();
            let fill_cards = select_resting(db, lesson_id, now, fill_count)?;
            cards.extend(fill_cards);
            cards.shuffle(&mut rand::rng());
        }
        Ok(cards)
    }
}

fn select_ready(db: &Connection, lesson_id: i64, now: f64) -> Result<Vec<Card>, StorageError> {
    let mut select_ready = db.prepare(
        r#"
SELECT
    id, prompt, reading, translation
FROM phrases
WHERE lesson_id = ?1 AND
    CASE
        WHEN learned_at IS NULL THEN 1
        WHEN learned_at < julianday(?2, 'start of day', '+3 hours') THEN 1
        WHEN content_changed_at > learned_at THEN 1
        ELSE 0
    END
    ORDER BY RANDOM() LIMIT ?3
;
"#,
    )?;
    let cards = select_ready
        .query_map(params![lesson_id, now, SESSION_SIZE], |row| {
            Ok(Card {
                id: row.get(0)?,
                goal: Goal::Learn,
                front: CardFront { kanji: row.get(1)? },
                back: CardBack {
                    yomi: row.get(2)?,
                    meaning: row.get(3)?,
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(cards)
}

fn select_resting(
    db: &Connection,
    lesson_id: i64,
    now: f64,
    limit: usize,
) -> Result<Vec<Card>, StorageError> {
    let mut select_resting = db.prepare(
        r#"
SELECT id, prompt, reading, translation
FROM phrases
WHERE lesson_id = ?1 AND
    NOT CASE
        WHEN learned_at IS NULL THEN 1
        WHEN learned_at < julianday(?2, 'start of day', '+3 hours') THEN 1
        WHEN content_changed_at > learned_at THEN 1
        ELSE 0
    END
    ORDER BY RANDOM() LIMIT ?3
"#,
    )?;
    let cards = select_resting
        .query_map(params![lesson_id, now, limit], |row| {
            Ok(Card {
                id: row.get(0)?,
                goal: Goal::Learn,
                front: CardFront { kanji: row.get(1)? },
                back: CardBack {
                    yomi: row.get(2)?,
                    meaning: row.get(3)?,
                },
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(cards)
}
