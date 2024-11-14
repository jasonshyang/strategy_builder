use chrono::{DateTime, Utc};

pub struct Transaction {
    pub date: DateTime<Utc>,
    pub amount: f64,
}

impl Transaction {
    pub fn new(date: DateTime<Utc>, amount: f64) -> Self {
        Self {
            date,
            amount,
        }
    }
}