use std::fmt;
use std::io;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum VehicleShadowError {
    Io(io::Error),
    Serialization(String),
    Database(String),
    NotFound(String),
    InvalidInput(String),
    Configuration(String),
    Rpc(String),
    Network(String),
}

impl fmt::Display for VehicleShadowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VehicleShadowError::Io(e) => write!(f, "IO error: {}", e),
            VehicleShadowError::Serialization(e) => write!(f, "Serialization error: {}", e),
            VehicleShadowError::Database(e) => write!(f, "Database error: {}", e),
            VehicleShadowError::NotFound(e) => write!(f, "Not found: {}", e),
            VehicleShadowError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            VehicleShadowError::Configuration(e) => write!(f, "Configuration error: {}", e),
            VehicleShadowError::Rpc(e) => write!(f, "RPC error: {}", e),
            VehicleShadowError::Network(e) => write!(f, "Network error: {}", e),
        }
    }
}

impl std::error::Error for VehicleShadowError {}

impl From<io::Error> for VehicleShadowError {
    fn from(err: io::Error) -> Self {
        VehicleShadowError::Io(err)
    }
}

impl From<bincode::error::EncodeError> for VehicleShadowError {
    fn from(err: bincode::error::EncodeError) -> Self {
        VehicleShadowError::Serialization(err.to_string())
    }
}

impl From<bincode::error::DecodeError> for VehicleShadowError {
    fn from(err: bincode::error::DecodeError) -> Self {
        VehicleShadowError::Serialization(err.to_string())
    }
}

impl From<sled::Error> for VehicleShadowError {
    fn from(err: sled::Error) -> Self {
        VehicleShadowError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for VehicleShadowError {
    fn from(err: serde_json::Error) -> Self {
        VehicleShadowError::Serialization(err.to_string())
    }
}

impl From<tonic::Status> for VehicleShadowError {
    fn from(err: tonic::Status) -> Self {
        VehicleShadowError::Rpc(err.to_string())
    }
}

impl From<tonic::transport::Error> for VehicleShadowError {
    fn from(err: tonic::transport::Error) -> Self {
        VehicleShadowError::Network(err.to_string())
    }
}

impl From<AddrParseError> for VehicleShadowError {
    fn from(err: AddrParseError) -> Self {
        VehicleShadowError::Configuration(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for VehicleShadowError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        VehicleShadowError::Configuration(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, VehicleShadowError>; 