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

