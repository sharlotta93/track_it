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

    fn update_date(&mut self, index: usize, new_date: String) {
        if self.user_records.len() >= index {
            self.user_records[index].modify_date(new_date);
        } else {
            println!("Record Not Found!!!");
        }
    }

    fn update_time(&mut self, index: usize, new_time: i64) {
        if self.user_records.len() >= index {
            self.user_records[index].modify_time(new_time);
        } else {
            println!("Record Not Found!!!");
        }
    }

    fn update_activity(&mut self, index: usize, new_activity: String) {
        if self.user_records.len() >= index {
            self.user_records[index].modify_activity(new_activity);
        } else {
            println!("Record Not Found!!!");
        }
    }
}

pub fn run() {
     let email: String = String::from("arekiert@op.pl");
     let username: String = String::from("aekiert");
     let mut user1 = User::build_user(email, username);

     let record1 = Record::create_new("date", 30, "activity");
     let record2 = Record::create_new("8/11", 15, "running");

     user1.add_record(record1);
     user1.add_record(record2);
     user1.update_date( 0, String::from("7/11"));
     user1.update_time( 0, 25);
     user1.update_activity( 0, String::from("coding"));
     println!("user is {:#?}", user1);
    // to do: implement date/time library Chrono(chrono has minutes, duration only sec ans nsec), display in front end
}