CREATE SCHEMA IF NOT EXISTS query_analytics;

CREATE TABLE IF NOT EXISTS query_analytics.query_events (
    idempotency_key TEXT PRIMARY KEY,
    normalized_query TEXT NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS query_analytics.query_frequencies (
    normalized_query TEXT PRIMARY KEY,
    frequency BIGINT NOT NULL CHECK (frequency > 0)
);

CREATE TABLE IF NOT EXISTS query_analytics.service_state (
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    revision BIGINT NOT NULL DEFAULT 0
);

INSERT INTO query_analytics.service_state (singleton, revision)
VALUES (TRUE, 0) ON CONFLICT (singleton) DO NOTHING;
