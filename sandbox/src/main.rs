use chrono::NaiveDateTime;

fn main() {
    let timestamp_millis: i64 = 1662921288; //Sunday, September 11, 2022 6:34:48 PM

    //change format from timestamp to date
    let date = NaiveDateTime::from_timestamp(timestamp_millis, 0);


    let date_string = "2022-02-20 18:34:48".to_string();


    //change format from date to timestamp
    let timestamp = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap().timestamp();

    println!("{}", date);

    println!("{}", timestamp);

    //get current timestamp
    let current_timestamp = chrono::Local::now().timestamp();

    //get current timestamp only date without time
    let current = chrono::Local::now().date().and_hms(0, 0, 0).timestamp();


    println!("{}", current_timestamp);

    println!("{}", current);



}