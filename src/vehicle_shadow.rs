use crate::signal;
use crate::error::{Result, VehicleShadowError};
use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};
use sled;

pub struct VehicleShadow {
    database: sled::Db,
    config: bincode::config::Configuration,
}

impl VehicleShadow {
    pub fn create() -> Result<VehicleShadow> {
        Ok(VehicleShadow {
            database: sled::Config::new().temporary(true).open()?,
            config: standard(),
        })
    }

    pub fn create_with_path<P: AsRef<std::path::Path>>(path: P) -> Result<VehicleShadow> {
        Ok(VehicleShadow {
            database: sled::Config::new().path(path).open()?,
            config: standard(),
        })
    }

    pub fn set_signal(&self, signal: signal::Signal) -> Result<()> {
        let encoded = encode_to_vec(&signal, self.config)?;
        self.database.insert(signal.path, encoded)?;
        Ok(())
    }

    pub fn get_signal(&self, path: String) -> Result<signal::Signal> {
        let query_result = self.database.get(&path)?;
        if let Some(encoded_signal) = query_result {
            let (signal, _len): (signal::Signal, usize) =
                decode_from_slice(&encoded_signal, self.config)?;
            assert_eq!(encoded_signal.len(), _len);
            return Ok(signal);
        }

        Err(VehicleShadowError::NotFound(format!("Signal not found: {}", path)))
    }

    pub fn list_signals(&self) -> Result<Vec<String>> {
        let mut paths = Vec::new();
        for item in self.database.iter() {
            let (key, _) = item?;
            if let Ok(path) = String::from_utf8(key.to_vec()) {
                paths.push(path);
            }
        }
        Ok(paths)
    }

    pub fn delete_signal(&self, path: &str) -> Result<()> {
        self.database.remove(path)?;
        Ok(())
    }

    pub fn dump(&self) -> Result<()> {
        for item in self.database.iter() {
            let (key, value) = item?;
            let key_str = String::from_utf8_lossy(&key);
            let (signal, _len): (signal::Signal, usize) = decode_from_slice(&value, self.config)?;
            println!("key: {:?}, value: {:?}", key_str, signal);
        }
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.database.clear()?;
        Ok(())
    }
}
