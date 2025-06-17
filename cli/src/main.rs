use clap::{Parser, Subcommand};
use anyhow::Result;
use log::{info, error};
use std::collections::HashMap;

use tokio::io::unix::AsyncFdTryNewError;
use vehicle_shadow_client::{
    VehicleShadowClient, format_signal, format_value, parse_state_from_json,
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
    body_server_endpoint: String,

    #[arg(short, long, default_value = "http://[::1]:50052")]
    cabin_server_endpoint: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DomainType {
    Body,
    Cabin,
    Invalid,
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
    
    let mut body_client = VehicleShadowClient::connect(&cli.body_server_endpoint).await?;
    let mut cabin_client = VehicleShadowClient::connect(&cli.cabin_server_endpoint).await?;

    let mut clients: HashMap<DomainType, &mut VehicleShadowClient> = HashMap::new();
    clients.insert(DomainType::Body, &mut body_client );
    clients.insert(DomainType::Cabin, &mut cabin_client );

    match cli.command {
        Commands::Get { paths } => {
            get_signals(&mut clients, paths).await?;
        }
        Commands::Set { path, value } => {
            set_signal(&mut clients, path, value).await?;
        }
        Commands::Subscribe { paths } => {
            subscribe_signals(&mut clients, paths).await?;
        }
        Commands::Unsubscribe { paths } => {
            unsubscribe_signals(&mut clients, paths).await?;
        }
    }

    Ok(())
}

async fn get_signals(clients: &mut HashMap<DomainType, &mut VehicleShadowClient>, paths: Vec<String>) -> Result<()> {
    let classified_paths = classify_paths(paths);
    for (domain, paths ) in classified_paths {
        let domain_client: &mut &mut VehicleShadowClient = &mut clients.get_mut(&domain).unwrap(); // TODO: use ok_or_else
        let response = domain_client.get_signals(paths).await?;
        if response.success {
            println!("Successfully retrieved {} signals:", response.signals.len());
            for signal in response.signals {
                println!("{}", format_signal(&signal));
            }
        } else {
            error!("Failed to get signals: {}", response.error_message);
        }
    }
    
    Ok(()) // TODO: return Error if failed
}

async fn set_signal(clients: &mut HashMap<DomainType, &mut VehicleShadowClient>, path: String, value_json: String) -> Result<()> {
    let domain = get_target_domain(&path);
    let state = parse_state_from_json(&value_json).unwrap();
    let signals = [(path.clone(), state)].to_vec();
    let response = clients.get_mut(&domain).unwrap().set_signals(signals).await?;

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

async fn subscribe_signals(clients: &mut HashMap<DomainType, &mut VehicleShadowClient>, paths: Vec<String>) -> Result<()> {

    let classified_paths = classify_paths(paths);
    
    if classified_paths.is_empty() {
        println!("No valid paths to subscribe to");
        return Ok(());
    }

    println!("Subscribed to signals. Waiting for updates...");
    println!("Press Ctrl+C to stop");

    let mut ctrl_c = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;

    // 各ドメインのストリームを個別に処理
    let mut stream_tasks = Vec::new();
    
    for (domain, domain_paths) in classified_paths {
        let client = clients.get_mut(&domain).unwrap();
        let mut stream = client.subscribe(domain_paths).await?;
        
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

async fn unsubscribe_signals(clients: &mut HashMap<DomainType, &mut VehicleShadowClient>, paths: Vec<String>) -> Result<()> {
    let classified_paths = classify_paths(paths);
    
    for (domain, domain_paths) in classified_paths {
        if !domain_paths.is_empty() {
            let response = clients.get_mut(&domain).unwrap().unsubscribe(domain_paths.clone()).await?;
            
            if response.success {
                println!("Successfully unsubscribed from {} signals", domain_paths.len());
            } else {
                error!("Failed to unsubscribe from {:?}: {}", domain, response.error_message);
                return Err(anyhow::anyhow!("Failed to unsubscribe from {:?}: {}", domain, response.error_message));
            }
        }
    }

    Ok(())
}

fn get_target_domain(path: &String) -> DomainType {
    return if path.starts_with("Vehicle.Body") {
        DomainType::Body
    } else if path.starts_with("Vehicle.Cabin") {
        DomainType::Cabin
    } else {
        DomainType::Invalid
    }
}

fn classify_paths(paths: Vec<String>) -> HashMap<DomainType, Vec<String>> {
    let mut body_paths = Vec::new();
    let mut cabin_paths = Vec::new();
    paths.iter().filter(|path| get_target_domain(path) == DomainType::Body ).for_each(|path| body_paths.push(path.clone()) );
    paths.iter().filter(|path| get_target_domain(path) == DomainType::Cabin ).for_each(|path| cabin_paths.push(path.clone()) );
    let mut paths_classified: HashMap<DomainType, Vec<String>> = HashMap::new();
    paths_classified.insert(DomainType::Body, body_paths );
    paths_classified.insert(DomainType::Cabin, cabin_paths );
    paths_classified 
}
