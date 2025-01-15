-- Your SQL goes here
CREATE TABLE companies (
    company_id INTEGER PRIMARY KEY AUTOINCREMENT,
    contact_email TEXT NOT NULL,
    created DATETIME NOT NULL,
    description TEXT NOT NULL,
    name TEXT NOT NULL,
    updated DATETIME NOT NULL,
    website TEXT NOT NULL
);