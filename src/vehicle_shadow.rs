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

    pub fn set_signal(&self, signal: signal::Signal, token: &Option<String>) -> Result<()> {
        let encoded = encode_to_vec(&signal, self.config)?;
        if let Some(_) = token {
            if self.get_signal(signal.path.clone())?.state.lock_uuid != token.clone() {
                return Err(VehicleShadowError::Database(format!("Authentication failed.")))
            };
        }

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

    pub fn try_locks(&self, paths: Vec<String>, lock_uuid: &String) -> Result<()>{
        let is_locked = paths.iter().fold(false, |ret, path|{
            if true == ret {
                return true;
            }
            return self.is_locked(path.clone()).unwrap_or(true);
        });
        if true == is_locked {
            return Err(VehicleShadowError::NotFound(format!("Some of signals are already locked: {:?}", paths)));
        }

        for path in paths {
            self.try_lock(path, lock_uuid)?;
        }
        Ok(())
    }

    pub fn try_lock(&self, path: String, lock_uuid: &String) -> Result<()>{
        let is_locked= self.is_locked(path.clone())?;
        if true == is_locked{
            return Err(VehicleShadowError::Database(format!("Signal already locked: {}", path)))
        }

        let query_result = self.database.get(&path)?;
        if let None = query_result {
            return Err(VehicleShadowError::NotFound(format!("Signal not found: {}", path)));
        }

        let encoded_signal = query_result.unwrap();
        let (mut signal, _len): (signal::Signal, usize) =
            decode_from_slice(&encoded_signal, self.config)?;
        assert_eq!(encoded_signal.len(), _len);
        signal.state.lock_uuid = Some(lock_uuid.clone());
        self.set_signal(signal, &None)
    }

    pub fn release_lock(&self, lock_uuid: &String) -> Result<()>{
        for item in self.database.iter() {
            let (_, value) = item?;
            let (mut signal, _len): (signal::Signal, usize) = decode_from_slice(&value, self.config)?;
            if Some(lock_uuid.clone()) == signal.state.lock_uuid {
                signal.state.lock_uuid = None;
            }
            self.set_signal(signal, &None)?;
        }
        Ok(())
    }

    pub fn is_locked(&self, path: String) -> Result<bool>{
        let query_result = self.database.get(&path)?;
        if let None = query_result {
            return Err(VehicleShadowError::NotFound(format!("Signal not found: {}", path)));
        }

        let encoded_signal = query_result .unwrap();
        let (signal, _len): (signal::Signal, usize) =
            decode_from_slice(&encoded_signal, self.config)?;
        assert_eq!(encoded_signal.len(), _len);
        return if let Some(_) = signal.state.lock_uuid {
            Ok(true)
        } else {
            Ok(false)
        }
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
