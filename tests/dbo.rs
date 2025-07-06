// Test import incomes from the UniversalBank csv format.

use std::fs::File;

use chrono::NaiveDateTime;
use dbo_csv::{csv::DboCsvError, record::DboRecord};

fn test_record(date: &str, amount: f64) -> DboRecord {
    let income_date = NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").unwrap();
    DboRecord::from_date_and_amount(income_date, amount)
}

#[test]
fn import_all_from_csv() {
    let balance_file = File::open("tests/test_files/balance.csv").unwrap();
    let statement = dbo_csv::deserialize_statement(balance_file).unwrap();

    assert_eq!(4, statement.len());
    let records: Vec<_> = statement.iter().cloned().collect();
    assert_eq!(
        records,
        &[
            test_record("18.01.2024 12:36:00", 3302.00),
            test_record("05.02.2024 15:18:00", 265654.00),
            test_record("05.03.2024 14:20:00", 269359.00),
            test_record("05.04.2024 14:11:00", 275674.00),
        ]
    );
}

#[test]
fn fail_parsing_invalid_date() {
    let statement_file = File::open("tests/test_files/incorrect-date.csv").unwrap();
    let statement = dbo_csv::deserialize_statement(statement_file);
    assert!(statement.is_err());
    let error = statement.err().unwrap();
    match error {
        DboCsvError::InvalidRecord {
            row_number,
            message,
        } => {
            assert_eq!(
                row_number, 3,
                "wrong row number. incorrect date format is in row 3"
            );
            assert!(
                message.contains("invalid operation date format"),
                "{}",
                format!("error message should inform about invalid date, but it says: {message}")
            );
        }
    }
}
