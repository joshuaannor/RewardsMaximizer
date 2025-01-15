-- Your SQL goes here
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    created DATETIME NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    updated DATETIME NOT NULL,
    username TEXT NOT NULL UNIQUE
);