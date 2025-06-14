pub mod signal;
pub mod vehicle_shadow;
pub mod vss_json_loader;

use clap::Parser;
use log::{error, info};
use std::time::Duration;
use tokio;

#[derive(Parser, Debug)]
#[command(
    name = "vehicle-signal-shadow",
    version,
    about = "A vehicel shadow signal service"
)]
struct Cli {
    #[arg(short, long)]
    vss: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    env_logger::init();
    info!("vehicle-signal-shadow service started");
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install signal handler");
        info!("shutdown signal received");
    };

    let result = vss_json_loader::load_vss_json(args.vss);
    if let Err(e) = &result {
        println!("{}", e.to_string());
    }
    let signals = result.unwrap();
    let vehicle_shadow = vehicle_shadow::VehicleShadow::create().unwrap();
    for signal in signals {
        vehicle_shadow.set_signal(signal);
    }

    let _ = vehicle_shadow.dump();
    return;

    let main_loop = async {
        loop {
            if let Err(e) = do_work().await {
                error!("work failed: {}", e);
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    };

    tokio::select! {
        _ = main_loop => {},
        _ = shutdown_signal => {
            info!("gracefully shutting down");
        }
    }

    cleanup().await;
}

async fn do_work() -> Result<(), Box<dyn std::error::Error>> {
    info!("doing work...");
    Ok(())
}

async fn cleanup() {
    info!("cleaning up resources...");
}

fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn run() {
    let vehicle_shadow = vehicle_shadow::VehicleShadow::create().unwrap();
    let path: String = String::from("signal");
    let data = signal::Signal {
        path: path.clone(),
        state: signal::State {
            value: signal::Value::Bool(true),
            capability: true,
            availability: true,
            reserved: String::from("reserved"),
        },
        config: signal::Config {
            leaf_type: signal::LeafType::Actuator,
            deprecation: None,
            unit: None,
            min: None,
            max: None,
            description: None,
            comment: None,
            allowd: None,
            default: None,
        },
    };
    let _ = vehicle_shadow.set_signal(data);
    let signal = vehicle_shadow.get_signal(path);
    println!("{:?}", signal);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // TODO
    }
}
