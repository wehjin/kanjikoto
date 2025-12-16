CREATE TABLE users
(
    id         TEXT PRIMARY KEY,
    created_at TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_admin   INTEGER NOT NULL DEFAULT 0 CHECK (is_admin IN (0, 1))
);

CREATE TABLE lessons
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT NOT NULL,
    creator_id TEXT NOT NULL,
    FOREIGN KEY (creator_id) REFERENCES users (id) ON DELETE CASCADE
);
CREATE INDEX idx_lesson_creator ON lessons (creator_id);

CREATE TABLE phrases
(
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    lesson_id          INTEGER NOT NULL,
    prompt             TEXT    NOT NULL,
    reading            TEXT    NOT NULL,
    translation        TEXT    NOT NULL,
    content_changed_at REAL    NOT NULL DEFAULT (julianday('now', 'localtime')),
    learned_at         REAL             DEFAULT NULL,
    FOREIGN KEY (lesson_id) REFERENCES lessons (id) ON DELETE CASCADE
);
CREATE INDEX idx_phrase_lesson ON phrases (lesson_id);
