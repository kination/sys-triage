use crate::config::Config;
use anyhow::Result;
use crossterm::style::Stylize;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::Networks;
use log::info;
use byte_unit::UnitType;


pub async fn check(config: Arc<Config>) -> Result<()> {
    let (rx_th, tx_th) = (
        byte_unit::Byte::parse_str(&config.network.threshold_rx, true)?.as_u64(),
        byte_unit::Byte::parse_str(&config.network.threshold_tx, true)?.as_u64(),
    );

    let mut networks = Networks::new_with_refreshed_list();
    info!("Measuring Network I/O (1 second sample)...");
    
    // 비동기 sleep 사용 (메인 스레드 블로킹 방지)
    tokio::time::sleep(Duration::from_secs(1)).await;
    networks.refresh();

    info!("{:<15} {:<15} {:<15}", "INTERFACE", "RX/s", "TX/s");
    info!("{}", "-".repeat(45));

    for (interface_name, data) in &networks {
        let rx_speed = data.received();
        let tx_speed = data.transmitted();

        if rx_speed > rx_th || tx_speed > tx_th {
            let rx_str = byte_unit::Byte::from_u64(rx_speed).get_appropriate_unit(UnitType::Binary);
            let tx_str = byte_unit::Byte::from_u64(tx_speed).get_appropriate_unit(UnitType::Binary);

            let rx_disp = if rx_speed > rx_th {
                rx_str.to_string().red().bold()
            } else {
                rx_str.to_string().reset()
            };
            let tx_disp = if tx_speed > tx_th {
                tx_str.to_string().red().bold()
            } else {
                tx_str.to_string().reset()
            };

            info!("{:<15} {:<15} {:<15}", interface_name, rx_disp, tx_disp);
        }
    }
    Ok(())
}
