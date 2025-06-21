use crate::signal;

use std::fs;
use std::io;

const TAG_CHILDREN: &str = "children";
//const TAG_DESCRIPTION: &str = "description";
const TAG_TYPE: &str = "type";
const TAG_DATATYPE: &str = "datatype";
//const TAG_ALLOWED: &str = "allowed";
//const TAG_COMMENT: &str = "comment";
//const TAG_UNIT: &str = "unit";
const TAG_DEFAULT: &str = "default";
//const TAG_DEPRECATION: &str = "deprecation";
//const TAG_MIN: &str = "min";
//const TAG_MAX: &str = "max";

pub fn load_vss_json(
    vss_json_path: String,
) -> Result<Vec<signal::Signal>, Box<dyn std::error::Error>> {
    let vss_json_string = fs::read_to_string(vss_json_path).expect("Unable to read file");
    let vss_json_data: serde_json::Value =
        serde_json::from_str(&vss_json_string).expect("Unable to parse");

    let mut result: Vec<signal::Signal> = [].to_vec();
    if let serde_json::Value::Object(map) = vss_json_data {
        for (path, node) in map {
            load_branch(path, &node, &mut result)?;
        }
    } else {
        println!("Top-level JSON is not an object");
    }

    Ok(result)
}

fn load_branch(
    path: String,
    node: &serde_json::Value,
    result: &mut Vec<signal::Signal>,
) -> Result<(), Box<dyn std::error::Error>> {
    let children = read_children(node)?;
    for (child_key, child) in children {
        let child_leaf_type = read_type(&child)?;
        match child_leaf_type {
            signal::LeafType::Branch => {
                load_branch(path.clone() + "." + &child_key, &child, result)?
            }
            _ => load_leaf(path.clone() + "." + &child_key, &child, result)?,
        };
    }

    Ok(())
}

fn load_leaf(
    path: String,
    node: &serde_json::Value,
    result: &mut Vec<signal::Signal>,
) -> Result<(), Box<dyn std::error::Error>> {
    let signal = create_signal(path, &node)?;
    result.push(signal);
    Ok(())
}

fn read_children(
    node: &serde_json::Value,
) -> Result<serde_json::map::Map<String, serde_json::Value>, Box<dyn std::error::Error>> {
    let tag_children_value = node.get(TAG_CHILDREN);
    if None == tag_children_value {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "children not found",
        )));
    }

    let children = tag_children_value.unwrap();
    return if let serde_json::Value::Object(map) = children {
        Ok(map.clone())
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            "TODO: write error message",
        )))
    };
}

fn read_type(node: &serde_json::Value) -> Result<signal::LeafType, Box<dyn std::error::Error>> {
    let value_str = node.get(TAG_TYPE).and_then(|v| v.as_str()).ok_or_else(|| {
        Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("type not found: {}", node),
        ))
    })?;

    let leaf_type = value_str.parse()?;
    Ok(leaf_type)
}

fn read_datatype(
    node: &serde_json::Value,
) -> Result<signal::ValueType, Box<dyn std::error::Error>> {
    let data_type_value = node.get(TAG_DATATYPE);
    if None == data_type_value {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("datatype not found: {}", node),
        )));
    }

    let data_type_str = data_type_value.unwrap().to_string().replace("\"", "");
    let value_type: signal::ValueType = data_type_str.parse()?;
    Ok(value_type)
}

fn read_default_value(node: &serde_json::Value) -> Option<signal::Value> {
    let default_value = node.get(TAG_DEFAULT);
    if default_value.is_none() {
        return None;
    }

    let value_type = read_datatype(node);
    if let Err(_) = value_type {
        return None;
    }
    let value_type = value_type.unwrap();
    let value = default_value.unwrap();
    let default_value = value_type.build_value(value);
    Some(default_value)
}

fn create_signal(
    path: String,
    node: &serde_json::Value,
) -> Result<signal::Signal, Box<dyn std::error::Error>> {
    let signal = signal::Signal {
        path: path,
        state: create_state(node)?,
        config: create_config(node)?,
    };
    Ok(signal)
}

fn create_state(node: &serde_json::Value) -> Result<signal::State, Box<dyn std::error::Error>> {
    let default_value = read_default_value(node).unwrap_or(signal::Value::NAN);
    let ret = signal::State {
        value: default_value,
        capability: false,
        availability: false,
        lock_uuid: None,
        reserved: String::from("reserved"),
    };
    Ok(ret)
}

fn create_config(node: &serde_json::Value) -> Result<signal::Config, Box<dyn std::error::Error>> {
    let leaf_type = read_type(&node)?;
    let value_type = read_datatype(node)?;
    let ret = signal::Config {
        leaf_type: leaf_type,
        data_type: value_type,
        deprecation: None,
        unit: None,
        min: None,
        max: None,
        description: None,
        comment: None,
        allowd: None,
        default: None,
        end_point: String::new(),
    };
    Ok(ret)
}
