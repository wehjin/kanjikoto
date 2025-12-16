pub fn now_localtime(db: &rusqlite::Connection) -> Result<f64, rusqlite::Error> {
    db.query_row("SELECT julianday('now','localtime')", [], |row| row.get(0))
}
