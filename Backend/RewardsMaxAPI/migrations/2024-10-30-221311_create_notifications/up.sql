-- Your SQL goes here
CREATE TABLE notifications (
    notification_id INTEGER PRIMARY KEY AUTOINCREMENT,
    created DATETIME NOT NULL,
    message TEXT NOT NULL,
    type TEXT NOT NULL,
    updated DATETIME NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);