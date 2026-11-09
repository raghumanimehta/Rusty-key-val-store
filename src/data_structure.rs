use std::collections::HashMap;
use thiserror::Error;
use crate::persistence::wal;
use crate::persistence::wal::{write_entry_to_wal, WalOperation};

pub struct KeyValStore {
    store: HashMap<String, String>
}

#[derive(Error, Debug)]

pub enum KeyValStoreError {
    #[error("the {key} was already int the store")]
    KeyAlreadyInStore { key: String },

    #[error("the {key} was not found")]
    KeyNotFoundError { key: String },

    #[error("Write to file failed for some reason")]
    Io(#[from] std::io::Error),


}

impl KeyValStore {
    pub fn new() -> KeyValStore {
        KeyValStore { store: HashMap::new() }
    }

    pub fn insert(&mut self, key: String, val: String) -> Result<(), KeyValStoreError> {
        if let Some(_stored_key) = self.store.get(&key) {
            Err(KeyValStoreError::KeyAlreadyInStore { key })
        } else {
            let key_copy = key.clone();
            let val_copy = val.clone();
            self.store.insert(key, val);
            write_entry_to_wal(WalOperation::INSERT, key_copy.as_str(), val_copy.as_str())?;
            Ok(())
        }
    }

    pub fn get(&self, key:String) -> Result<String, KeyValStoreError> {
        if let Some(val) = self.store.get(&key) {
            Ok(val.to_owned())
        } else {
            Err(KeyValStoreError::KeyNotFoundError { key })
        }
    }
    pub fn delete(&mut self, key: String) -> Result<(), KeyValStoreError> {
        if let Some(_removed_val) = self.store.remove(&key) {
            Ok(())
        } else {
            Err(KeyValStoreError::KeyNotFoundError { key })
        }
    }

    pub fn list(&self) -> Vec<String> {
        let collected_keys : Vec<&String> = self.store.keys().collect();
        let mut keys = Vec::new();
        for key in collected_keys {
            keys.push(key.to_owned());
        }
        keys
    }

    pub fn update(&mut self, key: String, new_val: String) -> Result<String, KeyValStoreError>
    {
        match self.store.get_mut(&key) {
            Some(val) => Ok(std::mem::replace(val, new_val)),
            None => Err(KeyValStoreError::KeyNotFoundError { key }),
        }
    }
}
#[cfg(test)]
mod tests;
