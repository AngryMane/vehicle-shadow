use bincode::{Decode, Encode};
use log::warn;
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
    pub data_type: ValueType,
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
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
    Double(f64),
    BoolArray(Vec<bool>),
    StringArray(Vec<String>),
    Int8Array(Vec<i8>),
    Int16Array(Vec<i16>),
    Int32Array(Vec<i32>),
    Int64Array(Vec<i64>),
    Uint8Array(Vec<u8>),
    Uint16Array(Vec<u16>),
    Uint32Array(Vec<u32>),
    Uint64Array(Vec<u64>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
}

#[derive(Encode, Decode, Serialize, Deserialize, Clone, Debug)]
pub enum ValueType {
    TypeNAN,
    TypeBool,
    TypeString,
    TypeInt8,
    TypeInt16,
    TypeInt32,
    TypeInt64,
    TypeUint8,
    TypeUint16,
    TypeUint32,
    TypeUint64,
    TypeFloat,
    TypeDouble,
    TypeBoolArray,
    TypeStringArray,
    TypeInt8Array,
    TypeInt16Array,
    TypeInt32Array,
    TypeInt64Array,
    TypeUint8Array,
    TypeUint16Array,
    TypeUint32Array,
    TypeUint64Array,
    TypeFloatArray,
    TypeDoubleArray,
}

impl ValueType {
    pub fn build_value(&self, value: &serde_json::Value) -> Value {
        return match self {
            ValueType::TypeNAN => Value::NAN,
            ValueType::TypeBool => {
                if let Ok(value) = serde_json::from_value::<bool>(value.clone()) {
                    Value::Bool(value)
                } else {
                    warn!(
                        "{} is set as boolean deafult, but can't interpret as boolen",
                        value
                    );
                    Value::Bool(false)
                }
            }
            ValueType::TypeString => {
                if let Ok(value) = serde_json::from_value::<String>(value.clone()) {
                    Value::String(value)
                } else {
                    warn!(
                        "{} is set as boolean deafult, but can't interpret as boolen",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt8 => {
                if let Ok(value) = serde_json::from_value::<i8>(value.clone()) {
                    Value::Int8(value)
                } else {
                    warn!("{} is set as i8 deafult, but can't interpret as i8", value);
                    Value::NAN
                }
            }
            ValueType::TypeInt16 => {
                if let Ok(value) = serde_json::from_value::<i16>(value.clone()) {
                    Value::Int16(value)
                } else {
                    warn!(
                        "{} is set as i16 deafult, but can't interpret as i16",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt32 => {
                if let Ok(value) = serde_json::from_value::<i32>(value.clone()) {
                    Value::Int32(value)
                } else {
                    warn!(
                        "{} is set as i32 deafult, but can't interpret as i32",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt64 => {
                if let Ok(value) = serde_json::from_value::<i64>(value.clone()) {
                    Value::Int64(value)
                } else {
                    warn!(
                        "{} is set as i64 deafult, but can't interpret as i64",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint8 => {
                if let Ok(value) = serde_json::from_value::<u8>(value.clone()) {
                    Value::Uint8(value)
                } else {
                    warn!("{} is set as u8 deafult, but can't interpret as u8", value);
                    Value::NAN
                }
            }
            ValueType::TypeUint16 => {
                if let Ok(value) = serde_json::from_value::<u16>(value.clone()) {
                    Value::Uint16(value)
                } else {
                    warn!(
                        "{} is set as u16 deafult, but can't interpret as u16",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint32 => {
                if let Ok(value) = serde_json::from_value::<u32>(value.clone()) {
                    Value::Uint32(value)
                } else {
                    warn!(
                        "{} is set as u32 deafult, but can't interpret as u32",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint64 => {
                if let Ok(value) = serde_json::from_value::<u64>(value.clone()) {
                    Value::Uint64(value)
                } else {
                    warn!(
                        "{} is set as u64 deafult, but can't interpret as u64",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeFloat => {
                if let Ok(value) = serde_json::from_value::<f32>(value.clone()) {
                    Value::Float(value)
                } else {
                    warn!(
                        "{} is set as f32 deafult, but can't interpret as f32",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeDouble => {
                if let Ok(value) = serde_json::from_value::<f64>(value.clone()) {
                    Value::Double(value)
                } else {
                    warn!(
                        "{} is set as f64 deafult, but can't interpret as f64",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeBoolArray => {
                if let Ok(value) = serde_json::from_value::<Vec<bool>>(value.clone()) {
                    Value::BoolArray(value)
                } else {
                    warn!(
                        "{} is set as Vec<bool> deafult, but can't interpret as Vec<bool>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeStringArray => {
                if let Ok(value) = serde_json::from_value::<Vec<String>>(value.clone()) {
                    Value::StringArray(value)
                } else {
                    warn!(
                        "{} is set as Vec<String> deafult, but can't interpret as Vec<String>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt8Array => {
                if let Ok(value) = serde_json::from_value::<Vec<i8>>(value.clone()) {
                    Value::Int8Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<i8> deafult, but can't interpret as Vec<i8>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt16Array => {
                if let Ok(value) = serde_json::from_value::<Vec<i16>>(value.clone()) {
                    Value::Int16Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<i16> deafult, but can't interpret as Vec<i16>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt32Array => {
                if let Ok(value) = serde_json::from_value::<Vec<i32>>(value.clone()) {
                    Value::Int32Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<i32> deafult, but can't interpret as Vec<i32>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeInt64Array => {
                if let Ok(value) = serde_json::from_value::<Vec<i64>>(value.clone()) {
                    Value::Int64Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<i64> deafult, but can't interpret as Vec<i64>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint8Array => {
                if let Ok(value) = serde_json::from_value::<Vec<u8>>(value.clone()) {
                    Value::Uint8Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<u8> deafult, but can't interpret as Vec<u8>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint16Array => {
                if let Ok(value) = serde_json::from_value::<Vec<u16>>(value.clone()) {
                    Value::Uint16Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<u16> deafult, but can't interpret as Vec<u16>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint32Array => {
                if let Ok(value) = serde_json::from_value::<Vec<u32>>(value.clone()) {
                    Value::Uint32Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<u32> deafult, but can't interpret as Vec<u32>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeUint64Array => {
                if let Ok(value) = serde_json::from_value::<Vec<u64>>(value.clone()) {
                    Value::Uint64Array(value)
                } else {
                    warn!(
                        "{} is set as Vec<u64> deafult, but can't interpret as Vec<u64>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeFloatArray => {
                if let Ok(value) = serde_json::from_value::<Vec<f32>>(value.clone()) {
                    Value::FloatArray(value)
                } else {
                    warn!(
                        "{} is set as Vec<f32> deafult, but can't interpret as Vec<f32>",
                        value
                    );
                    Value::NAN
                }
            }
            ValueType::TypeDoubleArray => {
                if let Ok(value) = serde_json::from_value::<Vec<f64>>(value.clone()) {
                    Value::DoubleArray(value)
                } else {
                    warn!(
                        "{} is set as Vec<f64> deafult, but can't interpret as Vec<f64>",
                        value
                    );
                    Value::NAN
                }
            }
        };
    }
}

impl FromStr for ValueType {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match s.to_lowercase().as_str() {
            "boolean" => Ok(ValueType::TypeBool),
            "string" => Ok(ValueType::TypeString),
            "int8" => Ok(ValueType::TypeInt8),
            "int16" => Ok(ValueType::TypeInt16),
            "int32" => Ok(ValueType::TypeInt32),
            "int64" => Ok(ValueType::TypeInt64),
            "uint8" => Ok(ValueType::TypeUint8),
            "uint16" => Ok(ValueType::TypeUint16),
            "uint32" => Ok(ValueType::TypeUint32),
            "uint64" => Ok(ValueType::TypeUint64),
            "float" => Ok(ValueType::TypeFloat),
            "double" => Ok(ValueType::TypeDouble),
            "boolean[]" => Ok(ValueType::TypeBoolArray),
            "string[]" => Ok(ValueType::TypeStringArray),
            "int8[]" => Ok(ValueType::TypeInt8Array),
            "int16[]" => Ok(ValueType::TypeInt16Array),
            "int32[]" => Ok(ValueType::TypeInt32Array),
            "int64[]" => Ok(ValueType::TypeInt64Array),
            "uint8[]" => Ok(ValueType::TypeUint8Array),
            "uint16[]" => Ok(ValueType::TypeUint16Array),
            "uint32[]" => Ok(ValueType::TypeUint32Array),
            "uint64[]" => Ok(ValueType::TypeUint64Array),
            "float[]" => Ok(ValueType::TypeFloatArray),
            "double[]" => Ok(ValueType::TypeDoubleArray),
            _ => Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, s))),
        }
    }
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
            Value::Int8(v) => write!(f, "Int8({})", v),
            Value::Int16(v) => write!(f, "Int16({})", v),
            Value::Int32(v) => write!(f, "Int32({})", v),
            Value::Int64(v) => write!(f, "Int64({})", v),
            Value::Uint8(v) => write!(f, "Uint8({})", v),
            Value::Uint16(v) => write!(f, "Uint16({})", v),
            Value::Uint32(v) => write!(f, "Uint32({})", v),
            Value::Uint64(v) => write!(f, "Uint64({})", v),
            Value::Float(v) => write!(f, "Float({})", v),
            Value::Double(v) => write!(f, "Double({})", v),
            Value::BoolArray(v) => write!(f, "BoolArray({:?})", v),
            Value::StringArray(v) => write!(f, "StringArray({:?})", v),
            Value::Int8Array(v) => write!(f, "Int8Array({:?})", v),
            Value::Int16Array(v) => write!(f, "Int16Array({:?})", v),
            Value::Int32Array(v) => write!(f, "Int32Array({:?})", v),
            Value::Int64Array(v) => write!(f, "Int64Array({:?})", v),
            Value::Uint8Array(v) => write!(f, "Uint8Array({:?})", v),
            Value::Uint16Array(v) => write!(f, "Uint16Array({:?})", v),
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
