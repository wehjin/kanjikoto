use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub mod backend;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = backend::connect(Some("kanjikoto.db"));
        info!("Database path: {}", conn.path().unwrap());
        conn
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub is_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub lesson_id: i64,
    pub title: String,
    pub creator_id: String,
}

#[get("/api/users")]
pub async fn users() -> Result<Vec<User>> {
    use crate::core::data::backend::get_users;
    let users = DB.with(|db| get_users(db));
    Ok(users)
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhraseView {
    pub phrase_id: i64,
    pub prompt: String,
    pub reading: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LessonView {
    pub lesson_id: i64,
    pub title: String,
    pub phrases: Vec<PhraseView>,
}

#[get("/api/lesson_view")]
pub async fn lesson_view() -> Result<LessonView> {
    use crate::core::data::backend::*;
    let lesson = DB
        .with(|db| read_user_lesson("admin", db))?
        .expect("Failed to fetch lesson");
    let lesson_view = LessonView {
        lesson_id: lesson.lesson_id,
        title: lesson.title,
        phrases: vec![],
    };
    Ok(lesson_view)
}
