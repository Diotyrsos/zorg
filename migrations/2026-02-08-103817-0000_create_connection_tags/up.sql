CREATE TABLE IF NOT EXISTS connection_tags (
    connection_id INTEGER NOT NULL,
    tag_id        INTEGER NOT NULL,
    PRIMARY KEY (connection_id, tag_id),
    FOREIGN KEY(connection_id) REFERENCES connections(id),
    FOREIGN KEY(tag_id) REFERENCES tags(id)
);

CREATE INDEX idx_connection_tags_connection
ON connection_tags(connection_id);

CREATE INDEX idx_connection_tags_tag
ON connection_tags(tag_id);
