use std::collections::HashMap;
use thiserror::Error;
use crate::persistence::wal;
pub struct KeyValStore {
    store: HashMap<String, String>
}

#[derive(Error, Debug)]

pub enum KeyValStoreError {
    #[error("the {key} was already int the store")]
    KeyAlreadyInStore { key: String },

    #[error("the {key} was not found")]
    KeyNotFoundError { key: String }

}

// pub enum FileError {
//     #[error("File operation failed: {0}")]
//     Io(#[from] std::io::Error),
// }

impl KeyValStore {
    pub fn new() -> KeyValStore {
        KeyValStore { store: HashMap::new() }
    }

    pub fn insert(&mut self, key: String, val: String) -> Result<(), KeyValStoreError> {
        if let Some(_stored_key) = self.store.get(&key) {
            Err(KeyValStoreError::KeyAlreadyInStore { key })
        } else {
            self.store.insert(key, val);
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
