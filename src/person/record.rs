#[derive(Debug)]
pub struct Record {
    pub date: String,
    pub time: String,
    pub activity: String,
}

impl Record {
    pub fn say_hi() {
        println!("Hello from record")
    }
}

pub fn create_new(date: &str, time: &str, activity: &str) -> Record {
    Record {
        date: String::from(date),
        time: String::from(time),
        activity: String::from(activity),
    }
}