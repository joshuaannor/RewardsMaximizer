-- Your SQL goes here
CREATE TABLE user_feedback (
    feedback_id INTEGER PRIMARY KEY AUTOINCREMENT,
    comments TEXT NOT NULL,
    company_id INTEGER NOT NULL,
    created DATETIME NOT NULL,
    rating INTEGER NOT NULL,
    updated DATETIME NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies (company_id),
    FOREIGN KEY (user_id) REFERENCES users (user_id)
);