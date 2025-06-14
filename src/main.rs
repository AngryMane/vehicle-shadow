mod rpc;
pub mod signal;
pub mod vehicle_shadow;
pub mod vss_json_loader;

use clap::Parser;
use log::info;
use tokio;

use crate::rpc::databroker_server::run_server;
use crate::vehicle_shadow::VehicleShadow;

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
    info!("vehicle-signal-shadow service started");
    let vehicle_shadow = initalize();

    let main_loop = async {
        let _ = run_server(vehicle_shadow, "[::1]:50051").await;
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
}

fn initalize() -> VehicleShadow {
    let args = Cli::parse();
    env_logger::init();

    let result = vss_json_loader::load_vss_json(args.vss);
    if let Err(e) = &result {
        println!("{}", e.to_string());
    }
    let signals = result.unwrap();
    let vehicle_shadow = vehicle_shadow::VehicleShadow::create().unwrap();
    for signal in signals {
        let _ = vehicle_shadow.set_signal(signal);
    }
    //let _ = vehicle_shadow.dump();
    vehicle_shadow
}

async fn cleanup() {
    info!("cleaning up resources...");
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        // TODO
    }
}
