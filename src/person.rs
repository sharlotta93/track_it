mod record;

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
//      current_records: vec![&Record]
// }
//
// impl CollectionOfRecords {
//     fn add_record(&self, record: &Record) -> Vector {
//         self.current_records.push(record)
//     }
// }

// cloning for now - but ideally it will be copy not clone, clone can be expensive
impl User {
    // fn get_username(&self) -> String {
    //     self.username.clone()
    // }
}

pub fn run() {
    // let email: String = String::from("arekiert@op.pl");
    // let username: String = String::from("aekiert");
    //
    // let user1 = build_user(email, username);
    //
    // println!("user is {:#?}", user1);
    // let record :Record = {"12/08","00:30", "Writing"}
    //
    // println!{"{:#?}", record}

}