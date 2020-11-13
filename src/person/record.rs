use std::time::Duration;

#[derive(Debug)]
pub struct Record {
    pub date: String,
    pub time: Duration,
    pub activity: String,
}

impl Record {
    pub fn create_new(date: &str, time: u64, activity: &str) -> Record {
        Record {
            date: String::from(date),
            time: Duration::from_secs (time),
            activity: String::from(activity),
        }
    }

    pub fn modify_date(&mut self, new: String) {
        self.date = new;
    }

    pub fn modify_time(&mut self, new: Duration) {
        self.time = new;
    }

    pub fn modify_activity(&mut self, new: String) {
        self.activity = new;
    }
}


