-- Your SQL goes here
CREATE TABLE user_rewards (
    user_reward_id INTEGER PRIMARY KEY AUTOINCREMENT,
    added DATETIME NOT NULL,
    expires_on DATETIME NOT NULL,
    reward_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    updated DATETIME NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (reward_id) REFERENCES rewards (reward_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);