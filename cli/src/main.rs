use clap::{Parser, Subcommand};
use anyhow::Result;
use log::{info, error};
use std::collections::HashMap;

use tokio::io::unix::AsyncFdTryNewError;
use vehicle_shadow_client::{
    format_signal, format_value, parse_state_from_json, parse_value_from_json, GetResponse, SetResponse, State, UnsubscribeResponse, VehicleShadowClient
};

#[derive(Parser)]
#[command(
    name = "vehicle-signal-shadow-cli",
    version,
    about = "CLI client for Vehicle Signal Shadow service"
)]
struct Cli {
    #[arg(short, long, default_value = "http://[::1]:50051")]
    body_server_endpoint: String,

    #[arg(short, long, default_value = "http://[::1]:50052")]
    cabin_server_endpoint: String,

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

    let mut client = VehicleShadowClient::create().await?;
    client.connect(&cli.body_server_endpoint, "Vehicle.Body".to_string()).await?;
    client.connect(&cli.cabin_server_endpoint, "Vehicle.Cabin".to_string()).await?;

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
    let response= client.get_signals(paths).await?;
    if response.success {
        println!("Successfully retrieved {} signals:", response.signals.len());
        for signal in response.signals {
            println!("{}", format_signal(&signal));
        }
    } else {
        error!("Failed to get signals: {}", response.error_message);
    }
    
    Ok(()) // TODO: return Error if failed
}

async fn set_signal(client: &mut VehicleShadowClient, path: String, value_json: String) -> Result<()> {
    let token = get_lock(client, path.clone()).await?;

    let state = parse_state_from_json(&value_json).unwrap_or_else(|_| { 
        println!("{}", value_json);
        let json_value: serde_json::Value = serde_json::from_str(&value_json).unwrap();
        println!("{}", json_value);
        let value = parse_value_from_json(&value_json).unwrap();
        State {
            value: Some(value),
            capability: None,
            availability: None,
            reserved: None,
        }
    });
    let response = client.set_signals([(path.clone(), state)].to_vec(), token.clone()).await?;
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

    let _ = release_lock(client, token).await?;

    Ok(())
}

async fn subscribe_signals(client: &mut VehicleShadowClient, paths: Vec<String>) -> Result<()> {
    println!("Subscribed to signals. Waiting for updates...");
    println!("Press Ctrl+C to stop");

    // 各シグナルのストリームを個別に処理(要議論)
    let mut stream_tasks = Vec::new();

    for path in paths {
        let mut stream = client.subscribe(path).await?;
        let task = tokio::spawn(async move {
            while let Ok(Some(response)) = stream.message().await {
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
        });
        stream_tasks.push(task);
    }

    // Ctrl+Cを待つか、すべてのタスクが完了するまで待つ
    let mut ctrl_c = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
    tokio::select! {
        _ = ctrl_c.recv() => {
            println!("\nReceived Ctrl+C, stopping subscription...");
        }
        _ = futures::future::join_all(stream_tasks) => {
            println!("All subscription streams ended");
        }
    }

    Ok(())
}

async fn unsubscribe_signals(client: &mut VehicleShadowClient, paths: Vec<String>) -> Result<()> {
    // TODO
    Ok(())
}

async fn get_lock(client: &mut VehicleShadowClient, path: String) -> Result<String> {
    let token = client.lock([path].to_vec()).await?;
    Ok(token.token)
}

async fn release_lock(client: &mut VehicleShadowClient, token: String) -> Result<()> {
    let _ = client.unlock(token).await?;
    Ok(())
}
