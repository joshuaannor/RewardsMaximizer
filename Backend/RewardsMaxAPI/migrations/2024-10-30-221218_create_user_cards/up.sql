-- Your SQL goes here
CREATE TABLE user_cards (
    user_card_id INTEGER PRIMARY KEY AUTOINCREMENT,
    added DATETIME NOT NULL,
    card_id INTEGER NOT NULL,
    expires_on DATETIME NOT NULL,
    updated DATETIME NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (card_id) REFERENCES cards (card_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);