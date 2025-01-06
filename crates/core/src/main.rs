// core/src/main.rs
use core::watcher::FuseWatcher;
use core::scanner::FuseScanner;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let monitor_path = "/path/to/monitor"; // 替换为实际路径
    let scan_interval_secs = 60; // 定时扫描间隔

    let mut watcher = FuseWatcher::new(monitor_path).await?;
    let scanner = FuseScanner::new(monitor_path, scan_interval_secs);

    let watcher_handle = task::spawn(async move {
        watcher.run().await;
    });

    let scanner_handle = task::spawn(async move {
        scanner.run().await;
    });

    watcher_handle.await?;
    scanner_handle.await?;

    Ok(())
}
