CREATE TABLE IF NOT EXISTS history (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER NOT NULL,
    started_at    INTEGER NOT NULL,
    ended_at      INTEGER NOT NULL,
    exit_code     TEXT NOT NULL,
    FOREIGN KEY(connection_id) REFERENCES connections(id)
);

CREATE INDEX idx_history_connection
ON history(connection_id);
