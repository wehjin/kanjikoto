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
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    prompt      TEXT    NOT NULL,
    reading     TEXT    NOT NULL,
    translation TEXT    NOT NULL,
    edited_at   TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    lesson_id   INTEGER NOT NULL,
    FOREIGN KEY (lesson_id) REFERENCES lessons (id) ON DELETE CASCADE
);
CREATE INDEX idx_phrase_lesson ON phrases (lesson_id);

CREATE TABLE scores
(
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    score     INTEGER NOT NULL DEFAULT 0,
    scored_at TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    phrase_id INTEGER NOT NULL,
    user_id   TEXT    NOT NULL,
    FOREIGN KEY (phrase_id) REFERENCES phrases (id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);
CREATE INDEX idx_score_phrase_user ON scores (phrase_id, user_id);