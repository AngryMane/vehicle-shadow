use crate::signal;
use bincode::config::standard;
use bincode::{decode_from_slice, encode_to_vec};
use sled;
use std::io;

pub struct VehicleShadow {
    database: sled::Db,
    config: bincode::config::Configuration,
}

impl VehicleShadow {
    pub fn create() -> Result<VehicleShadow, Box<dyn std::error::Error + Send + Sync>> {
        Ok(VehicleShadow {
            database: sled::Config::new().temporary(true).open()?,
            config: standard(),
        })
    }

    pub fn set_signal(
        &self,
        signal: signal::Signal,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let endoded = encode_to_vec(&signal, self.config)?;
        self.database.insert(signal.path, endoded)?;
        Ok(())
    }

    pub fn get_signal(
        &self,
        path: String,
    ) -> Result<signal::Signal, Box<dyn std::error::Error + Send + Sync>> {
        let query_result = self.database.get(path)?;
        if let Some(encoded_signal) = query_result {
            let (signal, _len): (signal::Signal, usize) =
                decode_from_slice(&encoded_signal, self.config)?;
            assert_eq!(encoded_signal.len(), _len);
            return Ok(signal);
        }

        Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "signal not found",
        )))
    }

    pub fn dump(&self) -> Result<(), Box<dyn std::error::Error>> {
        for item in self.database.iter() {
            let (key, value) = item?;
            let key_str = String::from_utf8_lossy(&key);
            let (signal, _len): (signal::Signal, usize) = decode_from_slice(&value, self.config)?;
            println!("key: {:?}, value: {:?}", key_str, signal);
        }
        Ok(())
    }
}
