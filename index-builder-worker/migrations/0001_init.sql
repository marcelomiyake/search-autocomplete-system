CREATE SCHEMA IF NOT EXISTS index_builder;

CREATE TABLE IF NOT EXISTS index_builder.snapshots (
    revision BIGINT PRIMARY KEY CHECK (revision >= 0),
    snapshot JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
