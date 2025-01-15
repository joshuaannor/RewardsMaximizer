-- Your SQL goes here
CREATE TABLE vendor_deals (
    deal_id INTEGER PRIMARY KEY AUTOINCREMENT,
    company_id INTEGER NOT NULL,
    created DATETIME NOT NULL,
    description TEXT NOT NULL,
    title TEXT NOT NULL,
    updated DATETIME NOT NULL,
    valid_from DATE NOT NULL,
    valid_to DATE NOT NULL,
    FOREIGN KEY (company_id) REFERENCES companies (company_id)
);