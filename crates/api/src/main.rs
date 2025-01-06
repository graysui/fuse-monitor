// crates/api/src/main.rs
use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建路由
    let app = Router::new()
        .route("/api/config", get(get_config).post(update_config))
        .route("/api/stats", get(get_stats))
        .route("/api/files", get(get_files));

    // 绑定地址并启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_config() -> Json<MonitorConfig> {
    // 假设从数据库或配置文件中获取配置
    Json(MonitorConfig {
        mount_path: "/path/to/monitor".to_string(),
        scan_interval: 60,
        enabled: true,
    })
}

async fn update_config(Json(config): Json<MonitorConfig>) -> &'static str {
    // 假设更新到数据库或配置文件
    "Config updated"
}

async fn get_stats() -> Json<MonitorStats> {
    // 假设从监控系统获取统计信息
    Json(MonitorStats {
        total_files: 1000,
        files_changed: 100,
    })
}

async fn get_files() -> Json<Vec<FileEntry>> {
    // 假设从监控系统获取文件列表
    Json(vec![
        FileEntry {
            path: "/path/to/file1".to_string(),
            status: "created".to_string(),
        },
        FileEntry {
            path: "/path/to/file2".to_string(),
            status: "modified".to_string(),
        },
    ])
}

#[derive(Serialize, Deserialize)]
struct MonitorConfig {
    mount_path: String,
    scan_interval: u64,
    enabled: bool,
}

#[derive(Serialize, Deserialize)]
struct MonitorStats {
    total_files: usize,
    files_changed: usize,
}

#[derive(Serialize, Deserialize)]
struct FileEntry {
    path: String,
    status: String,
}
