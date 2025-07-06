//! Import format for DBOsoft banking export
//!
//! The format is used by the popular online bank.
use std::io::{BufReader, Read};

use anyhow::Context;
use chrono::NaiveDateTime;
use csv::StringRecord;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

use crate::record::{DboRecord, DboStatement};

const DATE_COLUMN: usize = 4;
const AMOUNT_COLUMN: usize = 14;
const DESCRIPTION_COLUMN: usize = 15;

/// Reads incomes from DBOsoft-compatible CSV files.
/// The filter allows to pick incomes for particular date range.
pub fn read_statement<R>(reader: R) -> anyhow::Result<DboStatement>
where
    R: Read,
{
    let mut reader = DecodingReader::new(BufReader::new(reader), WINDOWS_1251.new_decoder());
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(&mut reader);
    let mut records = Vec::new();
    for result in csv_reader.records() {
        let record = result.context("failed to read record")?;
        let income = income_from_csv(&record)?;
        records.push(income);
    }

    Ok(DboStatement::new(records))
}

fn income_from_csv(record: &StringRecord) -> anyhow::Result<DboRecord> {
    let date = record
        .get(DATE_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("date not found"))?;
    let amount = record
        .get(AMOUNT_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("amount not found"))?;
    let comment = record
        .get(DESCRIPTION_COLUMN)
        .ok_or_else(|| anyhow::anyhow!("comment not found"))?;
    let date =
        NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").context("failed to parse date")?;
    let amount = amount.parse().context("failed to parse amount")?;
    Ok(DboRecord {
        date,
        amount,
        comment: comment.to_string(),
    })
}
