// Write ahead logs for preventing against crashes

/*
WAL format in a .txt file:

INSERT {key} {value}
UPDATE {key} {value}
DELETE {key} {value}

The WAL assumes that the operations have been validated by the caller. E.g. DELETE {key} {value} assumes
that the {key} exists in the database
 */
use std::arch::aarch64::__crc32b;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use thiserror::Error;

const WAL_PATH_STR: &str = "../WAL.txt";

pub enum WalOperation {
    INSERT,
    UPDATE,
    DELETE
}

impl WalOperation {
    fn to_string(&self) -> &str {
        match self {
            WalOperation::INSERT => "INSERT",
            WalOperation::UPDATE => "UPDATE",
            WalOperation::DELETE => "DELETE"
        }
    }
}

#[derive(Error, Debug)]

enum FileOpError {
    #[error("the file at {path} couldn't be opened")]
     FileOpenError{ path: String },

    #[error("the file at {path} couldn't be written")]
    FileWriteError { path: String }
}

pub fn write_entry_to_wal(op: WalOperation,
                          key: &str,
                          val: &str)  -> Result<(), Error>{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(WAL_PATH_STR)?;

    let op_string = op.to_string();
    writeln!(file, "{op_string} {key} {val}")?;
    file.sync_data()?; // immediately write to file
    Ok(())

}