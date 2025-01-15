-- Your SQL goes here
CREATE TABLE rewards (
    reward_id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id INTEGER NOT NULL,
    created DATETIME NOT NULL,
    description TEXT NOT NULL,
    name TEXT NOT NULL,
    updated DATETIME NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies (company_id)
);
