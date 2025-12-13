use crate::core::api::get_drills_url;
use dioxus::fullstack::Form;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
pub mod backend;
#[cfg(feature = "server")]
pub mod db;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPhrase {
    pub lesson_id: i64,
    pub prompt: String,
    pub reading: String,
    pub translation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Phrase {
    pub phrase_id: i64,
    pub lesson_id: i64,
    pub prompt: String,
    pub reading: String,
    pub translation: String,
}

#[get("/api/users")]
pub async fn users() -> Result<Vec<User>> {
    use db::prelude::*;
    let db = DB.lock().unwrap();
    let users = get_users(&db);
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
    use db::prelude::*;
    let db = DB.lock().unwrap();
    let lesson = read_user_lesson("admin", &db)?.expect("Should have hard-coded lesson");
    let phrases = read_phrases(lesson.lesson_id, &db)?
        .into_iter()
        .map(|p| PhraseView {
            phrase_id: p.phrase_id,
            prompt: p.prompt,
            reading: p.reading,
            meaning: p.translation,
        })
        .collect::<Vec<_>>();
    let lesson_view = LessonView {
        lesson_id: lesson.lesson_id,
        title: lesson.title,
        phrases,
    };
    Ok(lesson_view)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportCsvForm {
    pub lesson_id: i64,
    pub csv_url: String,
}

#[post("/api/import_csv")]
pub async fn import_csv(form: Form<ImportCsvForm>) -> Result<()> {
    use db::prelude::*;
    let lesson_id = form.0.lesson_id;
    let csv_url = form.0.csv_url.trim();
    let new_phrases = get_drills_url(csv_url)
        .await
        .into_iter()
        .map(|d| NewPhrase {
            lesson_id,
            prompt: d.kanji,
            reading: d.yomi,
            translation: d.meaning,
        })
        .collect::<Vec<_>>();

    let mut db = DB.lock().expect("Failed to lock database");
    insert_phrases(new_phrases, &mut db)?;
    Ok(())
}
