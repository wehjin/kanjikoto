use crate::core::data::User;
use std::path::Path;

pub fn connect(filename: Option<&'static str>) -> rusqlite::Connection {
    let (conn, existed) = if let Some(filename) = filename {
        let db_existed = Path::new(filename).exists();
        let conn = rusqlite::Connection::open(filename).expect("Failed to open database");
        (conn, db_existed)
    } else {
        let conn =
            rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
        (conn, false)
    };
    conn.pragma_update(None, "foreign_keys", "ON")
        .expect("Failed to enable foreign keys");

    if !existed {
        let up = include_str!("up.sql");
        conn.execute_batch(up)
            .expect("Failed to run database upgrade");

        let admin = User {
            id: "admin".to_string(),
            is_admin: true,
        };
        conn.execute(
            "INSERT INTO users (id, is_admin) VALUES (?1, ?2)",
            (&admin.id, &admin.is_admin),
        )
        .expect("Failed to create admin user");
    }
    conn
}

pub fn get_users(conn: &rusqlite::Connection) -> Vec<User> {
    let mut stmt = conn.prepare("SELECT id, is_admin FROM users").unwrap();
    let user_iter = stmt
        .query_map([], |row| {
            let id = row.get(0).unwrap();
            let is_admin = row.get(1).unwrap();
            Ok(User { id, is_admin })
        })
        .expect("Failed to fetch users");
    user_iter.collect::<Result<Vec<_>, _>>().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let conn = super::connect(None);
        let users = super::get_users(&conn);
        assert_eq!(users.len(), 1);
    }
}
