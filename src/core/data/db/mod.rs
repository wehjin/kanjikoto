use crate::core::backend;
use dioxus::prelude::info;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod prelude;

lazy_static! {
    pub static ref DB: Mutex<rusqlite::Connection> = {
        let conn = backend::connect(Some("kanjikoto.db"));
        info!("Database path: {}", conn.path().unwrap());
        Mutex::new(conn)
    };
}
