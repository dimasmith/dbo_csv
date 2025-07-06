//! Import format for DBOsoft banking export
//!
//! The format is used by the popular online bank.
use std::io::{BufReader, Read};

use anyhow::Context;
use chrono::NaiveDateTime;
use csv::StringRecord;
use encoding_rs::WINDOWS_1251;
use encoding_rs_rw::DecodingReader;

const DATE_COLUMN: usize = 4;
const AMOUNT_COLUMN: usize = 14;
const DESCRIPTION_COLUMN: usize = 15;

#[derive(Debug)]
pub struct DboStatement {
    records: Vec<DboRecord>,
}

#[derive(Debug, Clone)]
pub struct DboRecord {
    pub date: NaiveDateTime,
    pub amount: f64,
    pub comment: String,
}

impl PartialEq for DboRecord {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.amount == other.amount
    }
}

impl Eq for DboRecord {}

impl PartialOrd for DboRecord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DboRecord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

impl DboStatement {
    pub fn iter(&self) -> impl Iterator<Item = &DboRecord> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

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
    let mut incomes = Vec::new();
    for result in csv_reader.records() {
        let record = result.context("failed to read record")?;
        let income = income_from_csv(&record)?;
        incomes.push(income);
    }
    incomes.sort();

    Ok(DboStatement { records: incomes })
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
