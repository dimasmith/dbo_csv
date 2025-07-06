use chrono::NaiveDateTime;

/// A collection of DBO records.
/// Records in statement are always sorted by the operaton date in ascending order.
#[derive(Debug)]
pub struct DboStatement {
    records: Vec<DboRecord>,
}

/// A raw DBO record.
/// Built-in comparisons accounts only for entry date and amount.
#[derive(Debug, Clone)]
pub struct DboRecord {
    pub date: NaiveDateTime,
    pub amount: f64,
    pub comment: String,
}

impl DboStatement {
    pub fn new(mut records: Vec<DboRecord>) -> Self {
        records.sort();
        DboStatement { records }
    }
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
