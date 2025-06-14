use bincode::{Decode, Encode};
use serde;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io;
use std::str::FromStr;

#[derive(Encode, Decode, serde::Serialize, Deserialize, Debug, Clone)]
pub struct Signal {
    pub path: String,
    pub state: State,
    pub config: Config,
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub value: Value,
    pub capability: bool,
    pub availability: bool,
    pub reserved: String,
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub leaf_type: LeafType,
    pub deprecation: Option<String>,
    pub unit: Option<String>,
    pub min: Option<Value>,
    pub max: Option<Value>,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub allowd: Option<Vec<Value>>,
    pub default: Option<Value>,
}

#[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
pub enum Value {
    NAN,
    Bool(bool),
    String(String),
    Int32(i32),
    Int64(i64),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
    Double(f64),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
    Int32Array(Vec<i32>),
    Int64Array(Vec<i64>),
    Uint32Array(Vec<u32>),
    Uint64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
}

#[derive(Encode, Decode, Serialize, Deserialize, Debug, Clone)]
pub enum LeafType {
    Branch,
    Sensor,
    Attribute,
    Actuator,
}

impl FromStr for LeafType {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s.to_lowercase().as_str() {
            "branch" => Ok(LeafType::Branch),
            "sensor" => Ok(LeafType::Sensor),
            "attribute" => Ok(LeafType::Attribute),
            "actuator" => Ok(LeafType::Actuator),
            _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, s))),
        }
    }
}

//------------------------------------------------------------------------------------------------

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"path\": \"{}\", \"state\": {}, \"config\": {} }}",
            self.path, self.state, self.config
        )
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \"value\": \"{}\", \"capability\": {}, \"availability\": {}, \"reserved\": \"{}\" }}",
            self.value, self.capability, self.availability, self.reserved
        )
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::NAN => write!(f, "None"),
            Value::Bool(v) => write!(f, "Bool({})", v),
            Value::String(v) => write!(f, "String({})", v),
            Value::Int32(v) => write!(f, "Int32({})", v),
            Value::Int64(v) => write!(f, "Int64({})", v),
            Value::Uint32(v) => write!(f, "Uint32({})", v),
            Value::Uint64(v) => write!(f, "Uint64({})", v),
            Value::Float(v) => write!(f, "Float({})", v),
            Value::Double(v) => write!(f, "Double({})", v),
            Value::BoolArray(v) => write!(f, "BoolArray({:?})", v),
            Value::StringArray(v) => write!(f, "StringArray({:?})", v),
            Value::Int32Array(v) => write!(f, "Int32Array({:?})", v),
            Value::Int64Array(v) => write!(f, "Int64Array({:?})", v),
            Value::Uint32Array(v) => write!(f, "Uint32Array({:?})", v),
            Value::Uint64Array(v) => write!(f, "Uint64Array({:?})", v),
            Value::FloatArray(v) => write!(f, "FloatArray({:?})", v),
            Value::DoubleArray(v) => write!(f, "DoubleArray({:?})", v),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ \
            \"leaf_type\": \"{}\", \
            \"deprecation\": \"{:?}\", \
            \"unit\": \"{:?}\", \
            \"min\": \"{:?}\", \
            \"max\": \"{:?}\", \
            \"description\": \"{:?}\", \
            \"comment\": \"{:?}\", \
            \"allowd\": \"{:?}\", \
            \"default\": \"{:?}\" }}",
            self.leaf_type,
            self.deprecation,
            self.unit,
            self.min,
            self.max,
            self.description,
            self.comment,
            self.allowd,
            self.default
        )
    }
}

impl Display for LeafType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            LeafType::Branch => String::from("Branch"),
            LeafType::Sensor => String::from("Sensor"),
            LeafType::Attribute => String::from("Attribute"),
            LeafType::Actuator => String::from("Actuator"),
        };
        write!(f, "{}", name)
    }
}
