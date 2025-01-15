-- Your SQL goes here
CREATE TABLE comments (
    comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    comment_info TEXT NOT NULL,
    created DATETIME NOT NULL,
    entity_type TEXT NOT NULL,
    updated DATETIME NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);