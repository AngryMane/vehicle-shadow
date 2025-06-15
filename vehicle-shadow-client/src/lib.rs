//! Vehicle Signal Shadow Client Library
//! 
//! This library provides a high-level client interface for the Vehicle Signal Shadow gRPC service.
//! It includes utilities for connecting to the service, parsing JSON data, and formatting responses.

pub mod client;
pub mod error;
pub mod parser;
pub mod formatter;

// Re-export the generated proto types
pub mod vehicle_shadow {
    tonic::include_proto!("vehicle_shadow");
}

pub use client::VehicleShadowClient;
pub use error::{ClientError, Result};
pub use parser::{parse_state_from_json, parse_value_from_json};
pub use formatter::format_value;

// Re-export commonly used types
pub use vehicle_shadow::{
    GetRequest, GetResponse, SetRequest, SetResponse, SetSignalRequest, SetResult,
    SubscribeRequest, SubscribeResponse, UnsubscribeRequest, UnsubscribeResponse,
    Signal, State, Config, Value, ValueType, LeafType,
}; 