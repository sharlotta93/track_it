mod record;
use record::Record;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}

// struct CollectionOfRecords {
//     pub current_records: Vec<Record>
// }

// fn add_record(record: Record) -> Vec<Record> {
//     current_records.push(record);
// }

// impl CollectionOfRecords {
//     fn start_records() -> Vec<Record> {
//         let mut current_records :Vec<&Record> = Vec::new();
//         // println!("{:#?}", current_records);
//     }
// }

pub fn run() {
    // let email: String = String::from("arekiert@op.pl");
    // let username: String = String::from("aekiert");
    //
    // let user1 = build_user(email, username);
    //
    // println!("user is {:#?}", user1);

    let new_record = Record::create_new("date", "time", "activity");
    println!("{:#?}", new_record);
}