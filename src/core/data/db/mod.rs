use crate::core::backend;
use dioxus::fullstack::Lazy;
use dioxus::prelude::info;
use std::sync::Mutex;

pub mod prelude;

pub static DB: Lazy<Mutex<rusqlite::Connection>> = Lazy::new(|| async move {
    let conn = backend::connect(Some("kanjikoto.db"));
    info!("Database path: {}", conn.path().unwrap());
    dioxus::Ok(Mutex::new(conn))
});
