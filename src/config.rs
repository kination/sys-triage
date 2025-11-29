use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cpu: CpuConfig,
    pub disk: DiskConfig,
    pub network: NetworkConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuConfig {
    pub threshold_max: Option<f64>,
    pub threshold_min: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiskConfig {
    pub scan_paths: Vec<String>,
    pub size_threshold: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub threshold_rx: String,
    pub threshold_tx: String,
}

impl Config {
    pub async fn load(path: &str) -> Result<Self> {
        let content = tokio::fs::read(path)
            .await
            .context(format!("Failed to read config file: {}", path))?;
        let config: Config = serde_yaml::from_slice(&content)
            .context("Failed to parse config file")?;

        if config.cpu.threshold_max.is_none() && config.cpu.threshold_min.is_none() {
            anyhow::bail!("At least one of cpu.threshold_max or cpu.threshold_min must be set");
        }

        Ok(config)
    }
}
