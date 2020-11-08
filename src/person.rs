mod record;
use record::Record;


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

    // pub fn add_record(mut self, record: Record) -> User {
    //     self.user_records.push(record)
    // }
}




pub fn run() {
     let email: String = String::from("arekiert@op.pl");
     let username: String = String::from("aekiert");
     let user1 = User::build_user(email, username);

     println!("user is {:#?}", user1);

    // let new_record = Record::create_new("date", "time", "activity");
    // println!("{:#?}", new_record);
}