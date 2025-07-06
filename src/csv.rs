//! Import format for DBOsoft banking export
//!
//! The format is used by the popular online bank.
use std::io::{BufReader, Read};

use anyhow::bail;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

use crate::record::{DboRecord, DboStatement};

pub fn deserialize_statement<R>(reader: R) -> anyhow::Result<DboStatement>
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
    for r in record_iter {
        match r {
            Ok(record) => records.push(record),
            Err(err) => bail!(err),
        }
    }
    Ok(DboStatement::new(records))
}
