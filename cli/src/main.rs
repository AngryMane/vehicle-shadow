use clap::{Parser, Subcommand};
use tonic::transport::Channel;
use anyhow::Result;
use log::{info, error};

// 生成されたprotoファイルをインポート
pub mod vehicle_shadow {
    tonic::include_proto!("vehicle_shadow");
}

use vehicle_shadow::signal_service_client::SignalServiceClient;
use vehicle_shadow::{
    GetRequest, SetRequest, SetSignalRequest, SubscribeRequest, UnsubscribeRequest,
    Value, BoolArray, StringArray, Int32Array,
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
    
    let channel = Channel::from_shared(cli.server)?.connect().await?;
    let mut client = SignalServiceClient::new(channel);

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

async fn get_signals(client: &mut SignalServiceClient<Channel>, paths: Vec<String>) -> Result<()> {
    info!("Getting signals: {:?}", paths);
    
    let request = tonic::Request::new(GetRequest { paths });
    let response = client.get(request).await?;
    let response = response.into_inner();

    if response.success {
        println!("Successfully retrieved {} signals:", response.signals.len());
        for signal in response.signals {
            println!("Path: {}", signal.path);
            if let Some(state) = signal.state {
                if let Some(value) = state.value {
                    println!("  Value: {}", format_value(&value));
                }
                println!("  Capability: {}", state.capability);
                println!("  Availability: {}", state.availability);
            }
            if let Some(config) = signal.config {
                println!("  Type: {:?}", config.leaf_type);
                println!("  Data Type: {:?}", config.data_type);
                if let Some(unit) = config.unit {
                    println!("  Unit: {}", unit);
                }
                if let Some(description) = config.description {
                    println!("  Description: {}", description);
                }
            }
            println!();
        }
    } else {
        error!("Failed to get signals: {}", response.error_message);
        return Err(anyhow::anyhow!("Failed to get signals: {}", response.error_message));
    }

    Ok(())
}

async fn set_signal(client: &mut SignalServiceClient<Channel>, path: String, value_json: String) -> Result<()> {
    info!("Setting signal {} to value: {}", path, value_json);
    
    let value = parse_value_from_json(&value_json)?;
    
    let set_request = SetSignalRequest {
        path: path.clone(),
        value: Some(value),
    };

    let request = tonic::Request::new(SetRequest {
        signals: vec![set_request],
    });
    
    let response = client.set(request).await?;
    let response = response.into_inner();

    if response.success {
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

async fn subscribe_signals(client: &mut SignalServiceClient<Channel>, paths: Vec<String>) -> Result<()> {
    info!("Subscribing to signals: {:?}", paths);
    
    let request = tonic::Request::new(SubscribeRequest { paths });
    let mut stream = client.subscribe(request).await?.into_inner();

    println!("Subscribed to signals. Waiting for updates...");
    println!("Press Ctrl+C to stop");

    while let Some(response) = stream.message().await? {
        if let Some(signal) = response.signal {
            println!("Update for signal: {}", signal.path);
            if let Some(state) = signal.state {
                if let Some(value) = state.value {
                    println!("  Value: {}", format_value(&value));
                }
                println!("  Capability: {}", state.capability);
                println!("  Availability: {}", state.availability);
            }
            println!();
        }
        if !response.error_message.is_empty() {
            error!("Subscription error: {}", response.error_message);
        }
    }

    Ok(())
}

async fn unsubscribe_signals(client: &mut SignalServiceClient<Channel>, paths: Vec<String>) -> Result<()> {
    info!("Unsubscribing from signals: {:?}", paths);
    
    let request = tonic::Request::new(UnsubscribeRequest { paths });
    let response = client.unsubscribe(request).await?;
    let response = response.into_inner();

    if response.success {
        println!("Successfully unsubscribed from signals");
    } else {
        error!("Failed to unsubscribe: {}", response.error_message);
        return Err(anyhow::anyhow!("Failed to unsubscribe: {}", response.error_message));
    }

    Ok(())
}

fn parse_value_from_json(json_str: &str) -> Result<Value> {
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;
    
    match json_value {
        serde_json::Value::Bool(b) => Ok(Value {
            value: Some(vehicle_shadow::value::Value::BoolValue(b)),
        }),
        serde_json::Value::String(s) => Ok(Value {
            value: Some(vehicle_shadow::value::Value::StringValue(s)),
        }),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Ok(Value {
                        value: Some(vehicle_shadow::value::Value::Int32Value(i as i32)),
                    })
                } else {
                    Ok(Value {
                        value: Some(vehicle_shadow::value::Value::Int64Value(i)),
                    })
                }
            } else if let Some(f) = n.as_f64() {
                Ok(Value {
                    value: Some(vehicle_shadow::value::Value::DoubleValue(f)),
                })
            } else {
                Err(anyhow::anyhow!("Invalid number format"))
            }
        }
        serde_json::Value::Array(arr) => {
            // 配列の最初の要素の型を基に配列型を決定
            if arr.is_empty() {
                return Err(anyhow::anyhow!("Empty arrays are not supported"));
            }
            
            match &arr[0] {
                serde_json::Value::Bool(_) => {
                    let bool_values: Result<Vec<bool>, _> = arr.iter()
                        .map(|v| v.as_bool().ok_or_else(|| anyhow::anyhow!("Invalid boolean value")))
                        .collect();
                    Ok(Value {
                        value: Some(vehicle_shadow::value::Value::BoolArrayValue(BoolArray {
                            values: bool_values?,
                        })),
                    })
                }
                serde_json::Value::String(_) => {
                    let string_values: Result<Vec<String>, _> = arr.iter()
                        .map(|v| v.as_str().map(|s| s.to_string()).ok_or_else(|| anyhow::anyhow!("Invalid string value")))
                        .collect();
                    Ok(Value {
                        value: Some(vehicle_shadow::value::Value::StringArrayValue(StringArray {
                            values: string_values?,
                        })),
                    })
                }
                serde_json::Value::Number(_) => {
                    // 数値配列の場合はInt32Arrayとして扱う
                    let int_values: Result<Vec<i32>, _> = arr.iter()
                        .map(|v| v.as_i64().and_then(|i| i32::try_from(i).ok()).ok_or_else(|| anyhow::anyhow!("Invalid integer value")))
                        .collect();
                    Ok(Value {
                        value: Some(vehicle_shadow::value::Value::Int32ArrayValue(Int32Array {
                            values: int_values?,
                        })),
                    })
                }
                _ => Err(anyhow::anyhow!("Unsupported array element type")),
            }
        }
        _ => Err(anyhow::anyhow!("Unsupported JSON value type")),
    }
}

fn format_value(value: &Value) -> String {
    match &value.value {
        Some(vehicle_shadow::value::Value::BoolValue(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::StringValue(v)) => format!("\"{}\"", v),
        Some(vehicle_shadow::value::Value::Int8Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Int16Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Int32Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Int64Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Uint8Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Uint16Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Uint32Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::Uint64Value(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::FloatValue(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::DoubleValue(v)) => format!("{}", v),
        Some(vehicle_shadow::value::Value::BoolArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::StringArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Int8ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Int16ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Int32ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Int64ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Uint8ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Uint16ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Uint32ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::Uint64ArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::FloatArrayValue(v)) => format!("{:?}", v.values),
        Some(vehicle_shadow::value::Value::DoubleArrayValue(v)) => format!("{:?}", v.values),
        None => "NAN".to_string(),
    }
} 