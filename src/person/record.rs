#[derive(Debug)]
pub struct Record {
    pub date: String,
    pub time: String,
    pub activity: String,
}

impl Record {
    pub fn create_new(date: &str, time: &str, activity: &str) -> Record {
        Record {
            date: String::from(date),
            time: String::from(time),
            activity: String::from(activity),
        }
    }

    pub fn modify_date(&mut self, new: String) {
        self.date = new;
    }

    pub fn modify_time(&mut self, new: String) {
        self.time = new;
    }

    pub fn modify_activity(&mut self, new: String) {
        self.activity = new;
    }
}


