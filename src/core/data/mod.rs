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

#[get("/api/users")]
pub async fn users() -> Result<Vec<User>> {
    use crate::core::data::backend::get_users;
    let users = DB.with(|db| get_users(db));
    Ok(users)
}
