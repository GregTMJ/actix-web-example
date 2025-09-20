-- Add up migration script here
CREATE TABLE IF NOT EXISTS service_mock_responses (
    id SERIAL PRIMARY KEY,
    service_id INTEGER NOT NULL,
    data TEXT NOT NULL,
    data_hash VARCHAR NOT NULL
);
