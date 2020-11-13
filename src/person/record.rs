use chrono::{DateTime, TimeZone, NaiveDateTime, Utc, Duration};
use chrono::format::strftime;

#[derive(Debug)]
pub struct Record {
    pub date: String,
    pub time: Duration,
    pub activity: String,
}

impl Record {
    pub fn create_new(date: &str, time: i64, activity: &str) -> Record {
        Record {
            date: String::from(date),
            time: Duration::minutes(time),
            activity: String::from(activity),
        }
    }

    pub fn modify_date(&mut self, new: String) {
        self.date = new;
    }

    pub fn modify_time(&mut self, new: i64) {
        self.time = Duration::minutes(new);
    }

    pub fn modify_activity(&mut self, new: String) {
        self.activity = new;
    }
}


