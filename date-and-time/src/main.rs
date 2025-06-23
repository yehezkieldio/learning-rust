use chrono::{DateTime, NaiveDateTime};

fn main() {
    let rfc2822_string = "Fri, 28 Nov 2014 21:00:09 +0900";
    println!(
        "RFC 2822 fixed offset: {}",
        DateTime::parse_from_rfc2822(rfc2822_string).expect("Failed to parse RFC 2822 date")
    );

    let rfc3339_string = "2024-04-28T21:00:09+01:00";
    println!(
        "RFC 3339 fixed offset: {}",
        DateTime::parse_from_rfc3339(rfc3339_string).expect("Failed to parse RFC 3339 date")
    );

    let timestring = "2024-04-30 03:24:26.000 +0300";
    let naive_date_time = NaiveDateTime::parse_from_str(timestring, "%Y-%m-%d %H:%M:%S%.f %z")
        .expect("Failed to parse naive date time");
    println!("Naive date time with fixed offset: {}", naive_date_time);

    let timestring = "2024-04-30 03:24:26.000";
    let naive_date_time = NaiveDateTime::parse_from_str(&timestring, "%Y-%m-%d %H:%M:%S%.3f")?;
    println!("naive_date_time: {}", naive_date_time);
}
