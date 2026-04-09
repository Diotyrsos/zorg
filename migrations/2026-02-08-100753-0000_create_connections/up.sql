CREATE TABLE IF NOT EXISTS connections (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT NOT NULL,
    username      TEXT NOT NULL,
    hostname      TEXT NOT NULL,
    port          INTEGER,
    identity_file TEXT,
    note          TEXT,
    created_at    INTEGER NOT NULL DEFAULT (unixepoch()),
    modified_at   INTEGER
);
