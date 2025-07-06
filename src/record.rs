use chrono::NaiveDate;
use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;

/// A collection of DBO records.
/// Records in statement are always sorted by the operaton date in ascending order.
#[derive(Debug)]
pub struct DboStatement {
    records: Vec<DboRecord>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DboRecord {
    pub party_tax_id: String,
    pub party_bank_id: String,
    pub party_account: String,
    pub currency: String,
    #[serde(deserialize_with = "deserialize_operation_date_format")]
    pub operation_date: NaiveDateTime,
    pub operation_code: String,
    pub counterparty_bank_id: String,
    pub payment_provider: String,
    pub counterparty_account: String,
    pub counterparty_tax_id: String,
    pub counterparty_name: String,
    pub document_number: String,
    #[serde(deserialize_with = "deserialize_document_date_format")]
    pub document_date: NaiveDate,
    pub debit: Option<f64>,
    pub credit: Option<f64>,
    pub payment_purpose: String,
    pub coverage: f64,
}

impl DboStatement {
    pub fn new(mut records: Vec<DboRecord>) -> Self {
        records.sort();
        DboStatement { records }
    }

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

impl DboRecord {
    pub fn from_date_and_amount(operation_date: NaiveDateTime, amount: f64) -> Self {
        DboRecord {
            operation_date,
            coverage: amount,
            ..Default::default()
        }
    }
}

impl PartialEq for DboRecord {
    fn eq(&self, other: &Self) -> bool {
        self.operation_date == other.operation_date && self.coverage == other.coverage
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
        self.operation_date.cmp(&other.operation_date)
    }
}

const OPERATION_DATE_FORMAT: &str = "%d.%m.%Y %H:%M:%S";
const DOCUMENT_DATE_FORMAT: &str = "%d.%m.%Y";

fn deserialize_operation_date_format<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, OPERATION_DATE_FORMAT).map_err(|_| {
        let message = format!(
            "invalid operation date format [column 5]: expected format {OPERATION_DATE_FORMAT}, the date was {s}",
        );

        Error::custom(message)
    })?;
    Ok(dt)
}

fn deserialize_document_date_format<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDate::parse_from_str(&s, DOCUMENT_DATE_FORMAT).map_err(|_| {
        let message = format!(
            "invalid document date format [column 13]: expected format {DOCUMENT_DATE_FORMAT}, the date was {s}",
        );

        Error::custom(message)
    })?;
    Ok(dt)
}
