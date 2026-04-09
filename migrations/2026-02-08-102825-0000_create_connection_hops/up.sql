CREATE TABLE IF NOT EXISTS connection_hops (
    source_connection_id INTEGER NOT NULL,
    target_connection_id INTEGER NOT NULL,
    hop_order INTEGER NOT NULL,
    PRIMARY KEY (source_connection_id, target_connection_id, hop_order),
    FOREIGN KEY(source_connection_id) REFERENCES connections(id),
    FOREIGN KEY(target_connection_id) REFERENCES connections(id)
);

CREATE INDEX idx_connection_hops_source
ON connection_hops(source_connection_id);

CREATE INDEX idx_connection_hops_target
ON connection_hops(target_connection_id);
