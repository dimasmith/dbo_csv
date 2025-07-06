// Test import incomes from the UniversalBank csv format.

use std::fs::File;

use chrono::NaiveDateTime;
use dbo_csv::dbo::DboRecord;

fn record(date: &str, amount: f64) -> DboRecord {
    let income_date = NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").unwrap();
    DboRecord {
        date: income_date,
        amount,
        comment: "Default comment".to_string(),
    }
}

#[test]
fn import_all_from_csv() {
    let balance_file = File::open("tests/test_files/balance.csv").unwrap();
    let statement = dbo_csv::read_statement(balance_file).unwrap();

    assert_eq!(4, statement.len());
    let records: Vec<_> = statement.iter().cloned().collect();
    assert_eq!(
        records,
        &[
            record("18.01.2024 12:36:00", 3302.00),
            record("05.02.2024 15:18:00", 265654.00),
            record("05.03.2024 14:20:00", 269359.00),
            record("05.04.2024 14:11:00", 275674.00),
        ]
    );
}
