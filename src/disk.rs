use crate::config::Config;
use anyhow::Result;
use log::{info, error};
use crossterm::style::Stylize;
use byte_unit::UnitType;
use futures::future::BoxFuture;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::task::JoinSet;


fn scan_dir_recursive(
    path: PathBuf,
    threshold: u64,
    tx: mpsc::Sender<PathBuf>,
) -> BoxFuture<'static, ()> {
    Box::pin(async move {
        let mut join_set = JoinSet::new();

        if let Ok(mut entries) = fs::read_dir(&path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if let Ok(meta) = entry.metadata().await {
                    if meta.is_dir() {
                        let tx_clone = tx.clone();
                        join_set.spawn(scan_dir_recursive(path, threshold, tx_clone));
                    } else if meta.len() > threshold {
                        let _ = tx.send(path).await;
                    }
                }
            }
        }
        while join_set.join_next().await.is_some() {}
    })
}

fn get_disk_params(config: &Config) -> Option<(&Vec<String>, u64)> {
    let bytes = byte_unit::Byte::parse_str(&config.disk.size_threshold, true)
        .ok()?
        .as_u64();
    Some((&config.disk.scan_paths, bytes))
}

pub async fn check(config: Arc<Config>) -> Result<()> {
    let (paths, threshold) = match get_disk_params(&config) {
        Some(p) => p,
        None => return Ok(()),
    };

    info!("Scanning disk (Async) > {} bytes...", threshold);
    let (tx, mut rx) = mpsc::channel(100);

    for path in paths {
        let p = PathBuf::from(path);
        let t = tx.clone();
        tokio::spawn(scan_dir_recursive(p, threshold, t));
    }
    drop(tx);

    while let Some(file_path) = rx.recv().await {
        if let Ok(meta) = fs::metadata(&file_path).await {
            let size_str = byte_unit::Byte::from_u64(meta.len()).get_appropriate_unit(UnitType::Binary);
            info!("{} : {}", size_str.to_string().yellow(), file_path.display());
        }
    }
    Ok(())
}

pub async fn drop_files(config: Arc<Config>, extensions: Option<Vec<String>>) -> Result<()> {
    let (paths, threshold) = match get_disk_params(&config) {
        Some(p) => p,
        None => return Ok(()),
    };

    info!("Dropping files > {} bytes...", threshold);
    let (tx, mut rx) = mpsc::channel(100);

    for path in paths {
        let p = PathBuf::from(path);
        let t = tx.clone();
        tokio::spawn(scan_dir_recursive(p, threshold, t));
    }
    drop(tx);

    while let Some(file_path) = rx.recv().await {
        if let Some(exts) = &extensions {
            let ext = file_path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            if !exts.contains(&ext.to_string()) {
                continue;
            }
        }

        match fs::remove_file(&file_path).await {
            Ok(_) => info!("Deleted: {}", file_path.display()),
            Err(e) => error!("Failed to delete {}: {}", file_path.display(), e),
        }
    }
    Ok(())
}
