use index_builder_worker::{AppState, connect, initialize, router, run_builder};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();
    let pool = connect(&env::var("DATABASE_URL").expect("DATABASE_URL is required"))
        .await
        .expect("connect to PostgreSQL");
    initialize(&pool)
        .await
        .expect("initialize index builder schema");
    let analytics_url = env::var("QUERY_ANALYTICS_URL")
        .unwrap_or_else(|_| "http://query-analytics-service:8080".into());
    tokio::spawn(run_builder(pool.clone(), analytics_url));
    let addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind listener");
    tracing::info!(%addr, "index builder worker listening");
    axum::serve(listener, router(AppState { pool }))
        .await
        .expect("serve index builder");
}
