use thiserror::Error;
use crate::persistence::wal::{Entry, WalOperation};

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("WAL payload exceeds 65,535 bytes")]
    PayloadTooLong,
}

const WAL_ENTRY_MAX_BYTES: usize = u16::MAX as usize;

// Serializes the WAL payload.
pub fn serialize_entry(entry: Entry<'_>) -> Result<Vec<u8>, SerializationError> {
    let (op, val) = match entry.op {
        WalOperation::INSERT => (1, entry.val.as_bytes()),
        WalOperation::DELETE => (2, &[][..]),
        WalOperation::UPDATE => (3, entry.val.as_bytes()),
    };
    let key = entry.key.as_bytes();
    let payload_len = 3 + key.len() + val.len();
    if payload_len > WAL_ENTRY_MAX_BYTES {
        return Err(SerializationError::PayloadTooLong);
    }

    let mut serialized_entry = Vec::with_capacity(payload_len);
    serialized_entry.push(op);
    serialized_entry.extend_from_slice(&(key.len() as u16).to_le_bytes());
    serialized_entry.extend_from_slice(key);
    serialized_entry.extend_from_slice(val);
    Ok(serialized_entry)
}
