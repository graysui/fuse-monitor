// 示例定时扫描模块
use tokio::time::{self, Duration};
use walkdir::WalkDir;

pub struct FuseScanner {
    root_path: std::path::PathBuf,
    interval: Duration,
}

impl FuseScanner {
    pub fn new(path: impl Into<std::path::PathBuf>, interval_secs: u64) -> Self {
        Self {
            root_path: path.into(),
            interval: Duration::from_secs(interval_secs),
        }
    }

    pub async fn run(&self) {
        let mut interval = time::interval(self.interval);
        loop {
            interval.tick().await;
            self.scan().await;
        }
    }

    async fn scan(&self) {
        for entry in WalkDir::new(&self.root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            // 处理文件
        }
    }
}
