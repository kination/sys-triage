use crate::config::Config;
use log::info;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use anyhow::Result;
use crossterm::style::Stylize;
use rayon::prelude::*;



pub fn check(config: Arc<Config>) -> Result<()> {
    let mut sys = System::new_all();
    // First refresh to establish baseline
    sys.refresh_processes();
    
    info!("Measure CPU usage (1 second)...");
    std::thread::sleep(Duration::from_secs(1));
    
    // Second refresh to calculate delta
    sys.refresh_processes();

    let (max_th, min_th) = get_thresholds(&config);
    
    info!("{:<8} {:<8} {:<}", "PID", "CPU%", "COMMAND");
    info!("{}", "-".repeat(40));

    sys.processes()
        .par_iter()
        .filter(|(_, proc)| {
            let usage = proc.cpu_usage();
            info!("usage -> {:.4}", usage);
            usage > max_th || usage < min_th
        })
        .for_each(|(pid, proc)| {
            let usage = proc.cpu_usage();
            let color_usage = if usage > max_th {
                format!("{:.2}%", usage).red().bold()
            } else {
                format!("{:.2}%", usage).blue()
            };
            info!("{:<8} {:<8} {:<}", pid, color_usage, proc.name());
        });
    Ok(())
}

pub fn drop(config: Arc<Config>) -> Result<()> {
    let mut sys = System::new_all();
    std::thread::sleep(Duration::from_millis(500));
    sys.refresh_processes();

    let (max_th, min_th) = get_thresholds(&config);

    sys.processes().par_iter().for_each(|(pid, proc)| {
        let usage = proc.cpu_usage();
        if usage > max_th || usage < min_th {
            if proc.kill() {
                info!("Killed: {} (PID: {})", proc.name(), pid);
            }
        }
    });
    Ok(())
}

fn get_thresholds(config: &Config) -> (f32, f32) {
    (
        config.cpu.threshold_max.map(|v| v as f32).unwrap_or(f32::MAX),
        config.cpu.threshold_min.map(|v| v as f32).unwrap_or(f32::MIN),
    )
}
