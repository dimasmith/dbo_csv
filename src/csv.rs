//! Import format for DBOsoft banking export
//!
//! The format is used by the popular online bank.
use std::io::{BufReader, Read};

use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;
use thiserror::Error;

use crate::record::{DboRecord, DboStatement};

#[derive(Debug, Error)]
pub enum DboCsvError {
    #[error("failed to deserialize row {row_number} in the statement. the issue is {message}")]
    InvalidRecord { row_number: usize, message: String },
}

pub fn deserialize_statement<R>(reader: R) -> Result<DboStatement, DboCsvError>
where
    R: Read,
{
    let mut reader = DecodingReader::new(BufReader::new(reader), WINDOWS_1251.new_decoder());
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .has_headers(false)
        .from_reader(&mut reader);
    let record_iter = csv_reader.deserialize::<DboRecord>().skip(1);
    let mut records = vec![];
    for (idx, r) in record_iter.enumerate() {
        match r {
            Ok(record) => records.push(record),
            Err(err) => {
                return Err(DboCsvError::InvalidRecord {
                    row_number: idx + 2, // the DBO files have headers and numbering starts with 1
                    message: err.to_string(),
                });
            }
        }
    }
    Ok(DboStatement::new(records))
}
