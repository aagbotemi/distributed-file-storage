CREATE TABLE IF NOT EXISTS file_chunks (
    id SERIAL PRIMARY KEY,
    file_id UUID NOT NULL,
    chunk_id INTEGER NOT NULL,
    data BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (file_id, chunk_id)
);

CREATE INDEX idx_file_chunks_file_id ON file_chunks (file_id);