use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "triage")]
#[command(about = "Server resource diagnosis & cleanup tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, default_value = "triage.yaml")]
    pub config: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check resources
    Check {
        #[command(subcommand)]
        resource: ResourceType,
    },
    /// Drop/Kill resources
    Drop {
        #[command(subcommand)]
        resource: ResourceDropType,
    },
}

#[derive(Subcommand, Clone)]
pub enum ResourceType {
    Cpu,
    Disk,
    Network,
}

#[derive(Subcommand)]
pub enum ResourceDropType {
    Cpu,
    Disk {
        /// Filter by extension (e.g., --ext=zip,tar)
        #[arg(long, value_delimiter = ',')]
        ext: Option<Vec<String>>,
    },
}
