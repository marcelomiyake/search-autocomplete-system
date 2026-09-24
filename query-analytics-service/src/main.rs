use query_analytics_service::{AppState, connect, initialize, router, seed_sample_data};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let pool = connect(&database_url).await.expect("connect to PostgreSQL");
    initialize(&pool)
        .await
        .expect("initialize query analytics schema");
    if env::var("SEED_SAMPLE_DATA").as_deref() == Ok("true") {
        seed_sample_data(&pool)
            .await
            .expect("seed synthetic searches");
    }
    let addr = env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind listener");
    tracing::info!(%addr, "query analytics service listening");
    axum::serve(listener, router(AppState { pool }))
        .await
        .expect("serve query analytics service");
}
