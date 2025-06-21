mod config;
mod error;
mod rpc;
pub mod signal;
pub mod vehicle_shadow;
pub mod vss_json_loader;

use log::{error, info};
use tokio;

use crate::config::Config;
use crate::error::Result;
use crate::rpc::databroker_server::run_server;
use crate::vehicle_shadow::VehicleShadow;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env();
    config.setup_logging();
    
    info!("vehicle-signal-shadow service started");
    info!("Server address: {}", config.server_addr);
    info!("Log level: {}", config.log_level);
    
    let vehicle_shadow = initialize(&config)?;

    let main_loop = async {
        if let Err(e) = run_server(vehicle_shadow, &config.server_addr).await {
            error!("Server error: {}", e);
        }
    };

    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install signal handler");
        info!("shutdown signal received");
    };

    tokio::select! {
        _ = main_loop => {},
        _ = shutdown_signal => {
            info!("shutting down");
        }
    }

    cleanup().await;
    Ok(())
}

fn initialize(config: &Config) -> Result<VehicleShadow> {
    let signals = vss_json_loader::load_vss_json(config.vss.clone())?;
    let vehicle_shadow = VehicleShadow::create()?;
    
    for signal in signals {
        if let Err(e) = vehicle_shadow.set_signal(signal, &None) {
            error!("Failed to set signal: {}", e);
        }
    }
    
    Ok(vehicle_shadow)
}

async fn cleanup() {
    info!("cleaning up resources...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        // TODO: Add proper tests
        assert!(true);
    }
}
