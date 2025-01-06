// 示例 API 入口
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/api/config", get(get_config).post(update_config))
        .route("/api/stats", get(get_stats))
        .route("/api/files", get(get_files));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_config() -> &'static str {
    "Get Config"
}

async fn update_config() -> &'static str {
    "Update Config"
}

async fn get_stats() -> &'static str {
    "Get Stats"
}

async fn get_files() -> &'static str {
    "Get Files"
}
