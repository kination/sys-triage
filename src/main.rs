mod cli;
mod config;
mod cpu;
mod disk;
mod network;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;

// 모듈에서 필요한 enum들을 가져옵니다.
use cli::{Cli, Commands, ResourceDropType, ResourceType};
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logger with default level "info" if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    
    // 설정 로드
    let config = Config::load(&cli.config).await?;
    let config = Arc::new(config);

    // 커맨드 분기 처리
    match cli.command {
        Commands::Check { resource } => match resource {
            ResourceType::Cpu => cpu::check(config)?,
            ResourceType::Disk => disk::check(config).await?,
            ResourceType::Network => network::check(config).await?,
        },
        Commands::Drop { resource } => match resource {
            ResourceDropType::Cpu => cpu::drop(config)?,
            ResourceDropType::Disk { ext } => disk::drop_files(config, ext).await?,
        },
    }

    Ok(())
}
