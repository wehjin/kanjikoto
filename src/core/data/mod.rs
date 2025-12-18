use crate::core::api::get_drills_url;
use crate::core::data::card::Card;
use dioxus::prelude::*;
use lesson_status::LessonStatus;
use serde::{Deserialize, Serialize};

pub mod card;
#[cfg(feature = "server")]
pub mod db;
pub mod lesson_status;

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NewPhrase {
    pub prompt: String,
    pub reading: String,
    pub translation: String,
    pub content_changed_at: Option<f64>,
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Store)]
pub struct PhraseView {
    pub phrase_id: i64,
    pub prompt: String,
    pub reading: String,
    pub meaning: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Store)]
pub struct LessonView {
    pub lesson_id: i64,
    pub title: String,
    pub phrases: Vec<PhraseView>,
}
#[get("/api/lesson_view")]
pub async fn lesson_view() -> Result<Option<LessonView>> {
    use db::prelude::*;
    let db = DB.lock().expect("Failed to lock the database");
    if let Some(lesson) = read_user_lesson("admin", &db)? {
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
        Ok(Some(lesson_view))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ImportDetails {
    pub csv_url: String,
}

#[post("/api/import_csv")]
pub async fn import_csv(details: ImportDetails) -> Result<i64> {
    use crate::core::backend::insert_lesson::InsertLesson;
    use db::prelude::*;

    let csv_url = details.csv_url.trim();
    let drills = get_drills_url(csv_url).await;
    let phrases = drills
        .into_iter()
        .map(|d| NewPhrase {
            prompt: d.kanji,
            reading: d.yomi,
            translation: d.meaning,
            content_changed_at: None,
        })
        .collect::<Vec<_>>();
    let insert_lesson = InsertLesson {
        title: "Let this Grieving Soul Retire 1".to_string(),
        owner: "admin".to_string(),
        phrases,
    };
    let mut db = DB.lock().expect("Failed to lock the database");
    let lesson_id = insert_lesson.apply(&mut db)?;
    Ok(lesson_id)
}

#[server]
pub async fn query_lesson_status(lesson_id: i64) -> Result<LessonStatus> {
    use crate::core::backend::lesson::QueryLessonStatus;
    use crate::core::backend::misc::now_localtime;
    use db::prelude::*;
    let db = DB.lock().expect("Failed to lock the database");
    let now = now_localtime(&db)?;
    let status = QueryLessonStatus { lesson_id, now }.apply(&db)?;
    Ok(status)
}

#[server]
pub async fn query_practice_cards(lesson_id: i64) -> Result<Vec<Card>> {
    use crate::core::backend::lesson::QueryPracticeCards;
    use crate::core::backend::misc::now_localtime;
    use db::prelude::*;
    let db = DB.lock().expect("Failed to lock the database");
    let now = now_localtime(&db)?;
    let cards = QueryPracticeCards { lesson_id, now }.apply(&db)?;
    Ok(cards)
}

#[server]
pub async fn update_practice_cards(cards: Vec<Card>) -> Result<()> {
    use crate::core::backend::insert_lesson::UpdateLessonTimes;
    use crate::core::backend::misc::now_localtime;
    use db::prelude::*;
    let mut db = DB.lock().expect("Failed to lock the database");
    let now = now_localtime(&db)?;
    UpdateLessonTimes {
        phrase_ids: cards.iter().map(|c| c.id).collect(),
        now,
    }
    .apply(&mut db)?;
    Ok(())
}
