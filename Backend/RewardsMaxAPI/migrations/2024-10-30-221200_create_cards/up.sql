CREATE TABLE cards (
    card_id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    icon TEXT NOT NULL,
    color TEXT NOT NULL,
    created DATETIME NOT NULL,
    updated DATETIME NOT NULL,
    benefits TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'General',
    rating INTEGER,
    FOREIGN KEY (company_id) REFERENCES companies (company_id)
);