#[derive(Debug)]
struct Record {
    date: String,
    time: String,
    activity: String,
}

fn create_new_record(date: String, time: String, activity: String) -> Record {
    Record {
        date,
        time,
        activity,
    }
}

// struct CollectionOfRecords {
//      current_records: vec![]
// }

// impl CollectionOfRecords {
//     fn add_record(&self, record: &Record) -> Vector {
//         self.current_records.push(record)
//     }
// }

pub fn run() {
    let date: String = String::from( "12/08");
    let time: String = String::from("00:30");
    let activity: String = String::from("Writing");

    let new_record = create_new_record(date, time, activity);
    println!{"{:#?}", new_record}

    // CollectionOfRecords.add_record(new_record);
    // println!{"{:#?}", CollectionOfRecords};
}