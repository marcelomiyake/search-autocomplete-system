use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TermFrequency {
    pub term: String,
    pub frequency: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snapshot {
    pub revision: i64,
    pub terms: Vec<TermFrequency>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Suggestion {
    pub query: String,
    pub frequency: i64,
}

#[derive(Clone, Debug, Default)]
pub struct PrefixIndex {
    by_prefix: HashMap<String, Vec<Suggestion>>,
    pub revision: i64,
}

impl PrefixIndex {
    pub fn from_snapshot(snapshot: &Snapshot) -> Self {
        let mut terms = snapshot.terms.clone();
        terms.sort_by(|a, b| {
            b.frequency
                .cmp(&a.frequency)
                .then_with(|| a.term.cmp(&b.term))
        });
        let mut by_prefix: HashMap<String, Vec<Suggestion>> = HashMap::new();
        for item in terms {
            let suggestion = Suggestion {
                query: item.term.clone(),
                frequency: item.frequency,
            };
            for end in item.term.char_indices().map(|(i, c)| i + c.len_utf8()) {
                let prefix = item.term[..end].to_owned();
                let candidates = by_prefix.entry(prefix).or_default();
                if candidates.len() < 5 {
                    candidates.push(suggestion.clone());
                }
            }
        }
        Self {
            by_prefix,
            revision: snapshot.revision,
        }
    }

    pub fn suggest(&self, prefix: &str) -> Vec<Suggestion> {
        self.by_prefix.get(prefix).cloned().unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct AppState {
    index: Arc<RwLock<PrefixIndex>>,
    index_builder_url: String,
    http: reqwest::Client,
}

impl AppState {
    pub fn new(index_builder_url: String) -> Self {
        Self {
            index: Arc::new(RwLock::new(PrefixIndex::default())),
            index_builder_url,
            http: reqwest::Client::new(),
        }
    }

    pub async fn refresh(&self) -> Result<bool, reqwest::Error> {
        let response = self
            .http
            .get(format!(
                "{}/internal/v1/snapshots/latest",
                self.index_builder_url.trim_end_matches('/')
            ))
            .send()
            .await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(false);
        }
        let snapshot: Snapshot = response.error_for_status()?.json().await?;
        let mut current = self.index.write().await;
        if snapshot.revision <= current.revision {
            return Ok(false);
        }
        *current = PrefixIndex::from_snapshot(&snapshot);
        Ok(true)
    }

    pub async fn revision(&self) -> i64 {
        self.index.read().await.revision
    }

    pub async fn suggest(&self, prefix: &str) -> Vec<Suggestion> {
        self.index.read().await.suggest(prefix)
    }
}

#[derive(Deserialize)]
struct SuggestionsQuery {
    prefix: String,
}

#[derive(Serialize)]
struct SuggestionsResponse {
    prefix: String,
    suggestions: Vec<Suggestion>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

pub fn normalize_prefix(input: &str) -> Result<String, &'static str> {
    let normalized = input
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    if normalized.len() > 50 {
        return Err("prefix must be at most 50 characters");
    }
    if !normalized
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b == b' ')
    {
        return Err("prefix may contain only English letters and spaces");
    }
    Ok(normalized)
}

async fn suggestions(
    State(state): State<AppState>,
    Query(query): Query<SuggestionsQuery>,
) -> impl IntoResponse {
    let prefix = match normalize_prefix(&query.prefix) {
        Ok(prefix) => prefix,
        Err(error) => {
            return (StatusCode::BAD_REQUEST, Json(ErrorResponse { error })).into_response();
        }
    };
    if prefix.is_empty() {
        return (
            StatusCode::OK,
            Json(SuggestionsResponse {
                prefix,
                suggestions: vec![],
            }),
        )
            .into_response();
    }
    let index = state.index.read().await;
    (
        StatusCode::OK,
        Json(SuggestionsResponse {
            prefix: prefix.clone(),
            suggestions: index.suggest(&prefix),
        }),
    )
        .into_response()
}

async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let revision = state.revision().await;
    (
        StatusCode::OK,
        Json(serde_json::json!({"status":"ok", "snapshotRevision":revision})),
    )
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/suggestions", get(suggestions))
        .route("/healthz", get(health))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_english_input_and_whitespace() {
        assert_eq!(normalize_prefix("  New   York  ").unwrap(), "new york");
        assert_eq!(normalize_prefix("  ").unwrap(), "");
    }

    #[test]
    fn rejects_non_english_input_and_long_prefixes() {
        assert!(normalize_prefix("café").is_err());
        assert!(normalize_prefix(&"a".repeat(51)).is_err());
    }

    #[test]
    fn ranks_by_frequency_then_lexically_and_limits_to_five() {
        let snapshot = Snapshot {
            revision: 8,
            terms: vec![
                TermFrequency {
                    term: "apple pie".into(),
                    frequency: 10,
                },
                TermFrequency {
                    term: "apple tart".into(),
                    frequency: 10,
                },
                TermFrequency {
                    term: "apricot".into(),
                    frequency: 8,
                },
                TermFrequency {
                    term: "application".into(),
                    frequency: 7,
                },
                TermFrequency {
                    term: "apply".into(),
                    frequency: 6,
                },
                TermFrequency {
                    term: "appetite".into(),
                    frequency: 5,
                },
                TermFrequency {
                    term: "apple".into(),
                    frequency: 4,
                },
            ],
        };
        let index = PrefixIndex::from_snapshot(&snapshot);
        let results = index.suggest("app");
        assert_eq!(results.len(), 5);
        assert_eq!(results[0].query, "apple pie");
        assert_eq!(results[1].query, "apple tart");
        assert!(
            results
                .windows(2)
                .all(|pair| pair[0].frequency >= pair[1].frequency)
        );
        assert_eq!(index.revision, 8);
    }
}
