use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Hash)]
pub struct FileName {
    pub file_name: String,
    pub time_stamp: DateTime<Utc>,
}
