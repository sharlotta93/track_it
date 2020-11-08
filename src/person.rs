mod record;
use record::Record;
use std::vec::Vec;


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    user_records: Vec<Record>,
}

impl User {
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            user_records: Vec::new(),
        }
    }

    fn add_record(&mut self, record: Record) {
        self.user_records.push(record);
    }

    // fn change_date(&mut self, mut record: Record, new_date: String) {
    //         record.modify_date(new_date);
    // }
}




pub fn run() {
     let email: String = String::from("arekiert@op.pl");
     let username: String = String::from("aekiert");
     let mut user1 = User::build_user(email, username);

     println!("user is {:#?}", user1);

     let mut record1 = Record::create_new("date", "time", "activity");
     let mut record2 = Record::create_new("8/11", "12:00", "running");
     record1.modify_date(String::from("21/11"));
     record1.modify_time(String::from("14:00"));
     record1.modify_activity(String::from("painting"));

     user1.add_record(record1);
     user1.add_record(record2);

     //user1.change_date(record2, String::from("7/11"));
     println!("user is {:#?}", user1);
}