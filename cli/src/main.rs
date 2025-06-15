use clap::{Parser, Subcommand};
use anyhow::Result;
use log::{info, error};

use vehicle_shadow_client::{
    VehicleShadowClient, format_signal, format_value,
    GetResponse, SetResponse, UnsubscribeResponse,
};

#[derive(Parser)]
#[command(
    name = "vehicle-signal-shadow-cli",
    version,
    about = "CLI client for Vehicle Signal Shadow service"
)]
struct Cli {
    #[arg(short, long, default_value = "http://[::1]:50051")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get signal values by paths
    Get {
        /// Signal paths to get
        #[arg(required = true)]
        paths: Vec<String>,
    },
    /// Set signal values
    Set {
        /// Signal path
        #[arg(short, long, required = true)]
        path: String,
        /// Signal value (JSON format)
        #[arg(short, long, required = true)]
        value: String,
    },
    /// Subscribe to signal changes
    Subscribe {
        /// Signal paths to subscribe
        #[arg(required = true)]
        paths: Vec<String>,
    },
    /// Unsubscribe from signal changes
    Unsubscribe {
        /// Signal paths to unsubscribe
        #[arg(required = true)]
        paths: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    let mut client = VehicleShadowClient::connect(&cli.server).await?;

    match cli.command {
        Commands::Get { paths } => {
            get_signals(&mut client, paths).await?;
        }
        Commands::Set { path, value } => {
            set_signal(&mut client, path, value).await?;
        }
        Commands::Subscribe { paths } => {
            subscribe_signals(&mut client, paths).await?;
        }
        Commands::Unsubscribe { paths } => {
            unsubscribe_signals(&mut client, paths).await?;
        }
    }

    Ok(())
}

async fn get_signals(client: &mut VehicleShadowClient, paths: Vec<String>) -> Result<()> {
    let response = client.get_signals(paths).await?;

    if response.success {
        println!("Successfully retrieved {} signals:", response.signals.len());
        for signal in response.signals {
            println!("{}", format_signal(&signal));
        }
    } else {
        error!("Failed to get signals: {}", response.error_message);
        return Err(anyhow::anyhow!("Failed to get signals: {}", response.error_message));
    }

    Ok(())
}

async fn set_signal(client: &mut VehicleShadowClient, path: String, value_json: String) -> Result<()> {
    let response = client.set_signal(path.clone(), &value_json).await?;

    if response.success {
        println!("Successfully set signal: {}", path);
        for result in response.results {
            if result.success {
                println!("  {}: OK", result.path);
            } else {
                println!("  {}: ERROR - {}", result.path, result.error_message);
            }
        }
    } else {
        error!("Failed to set signal: {}", response.error_message);
        return Err(anyhow::anyhow!("Failed to set signal: {}", response.error_message));
    }

    Ok(())
}

async fn subscribe_signals(client: &mut VehicleShadowClient, paths: Vec<String>) -> Result<()> {
    let mut stream = client.subscribe(paths).await?;

    println!("Subscribed to signals. Waiting for updates...");
    println!("Press Ctrl+C to stop");

    while let Some(response) = stream.message().await? {
        if let Some(signal) = response.signal {
            println!("Update for signal: {}", signal.path);
            if let Some(state) = signal.state {
                if let Some(value) = state.value {
                    println!("  Value: {}", format_value(&value));
                }
                println!("  Capability: {}", state.capability.unwrap_or(false));
                println!("  Availability: {}", state.availability.unwrap_or(false));
            }
            println!();
        }
        if !response.error_message.is_empty() {
            error!("Subscription error: {}", response.error_message);
        }
    }

    Ok(())
}

async fn unsubscribe_signals(client: &mut VehicleShadowClient, paths: Vec<String>) -> Result<()> {
    let response = client.unsubscribe(paths).await?;

    if response.success {
        println!("Successfully unsubscribed from signals");
    } else {
        error!("Failed to unsubscribe: {}", response.error_message);
        return Err(anyhow::anyhow!("Failed to unsubscribe: {}", response.error_message));
    }

    Ok(())
} 