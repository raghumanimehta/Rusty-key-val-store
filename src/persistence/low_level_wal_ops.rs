use thiserror::Error;
use crate::persistence::wal::{Entry, WalOperation};

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("WAL payload exceeds 65,535 bytes")]
    PayloadTooLong,
}

const WAL_ENTRY_MAX_BYTES: usize = u16::MAX as usize;

// The payload length must fit in the record's u16 length field.
pub fn serialize_payload(entry: Entry<'_>) -> Result<(Vec<u8>, usize), SerializationError> {
    let (op, val) = match entry.op {
        WalOperation::INSERT => (1, entry.val.as_bytes()),
        WalOperation::DELETE => (2, &[][..]),
        WalOperation::UPDATE => (3, entry.val.as_bytes()),
    };
    let key = entry.key.as_bytes();
    // 1 byte for OP; 2 bytes for storing length of key
    let payload_len = 3 + key.len() + val.len();
    if payload_len > WAL_ENTRY_MAX_BYTES {
        return Err(SerializationError::PayloadTooLong);
    }

    let mut serialized_payload = Vec::with_capacity(payload_len);
    serialized_payload.push(op);
    serialized_payload.extend_from_slice(&(key.len() as u16).to_le_bytes());
    serialized_payload.extend_from_slice(key);
    serialized_payload.extend_from_slice(val);
    Ok((serialized_payload, payload_len))
}

pub fn prepare_record(entry: Entry<'_>) -> Result<(Vec<u8>, usize), SerializationError> {
    let (payload, payload_len) = serialize_payload(entry)?;
    // 2 bytes for payload length and 4 bytes for checksum
    let mut record = Vec::with_capacity(2 + payload_len + 4);
    record.extend_from_slice(&(payload_len as u16).to_le_bytes());
    record.extend_from_slice(&payload);
    let checksum = crc32fast::hash(&record);
    record.extend_from_slice(&checksum.to_le_bytes());
    let record_len = record.len();
    Ok((record, record_len))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_contains_length_payload_and_crc_in_wire_order() {
        let entry = Entry::new(WalOperation::INSERT, "k", "v");
        let (record, len) = prepare_record(entry).unwrap();

        assert_eq!(record, [5, 0, 1, 1, 0, b'k', b'v', 0x69, 0xd8, 0xe7, 0x75]);
        assert_eq!(len, record.len());
    }

    #[test]
    fn record_length_accepts_maximum_payload_and_rejects_larger_payload() {
        let value = "v".repeat(WAL_ENTRY_MAX_BYTES - 3);
        let (record, _) = prepare_record(Entry::new(WalOperation::INSERT, "", &value)).unwrap();
        assert_eq!(&record[..2], &[0xff, 0xff]);
        assert!(matches!(
            prepare_record(Entry::new(WalOperation::INSERT, "k", &value)),
            Err(SerializationError::PayloadTooLong)
        ));
    }
}
