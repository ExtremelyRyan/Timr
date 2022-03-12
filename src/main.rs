use chrono::{DateTime, Local, TimeZone};
use std::env;
use serde;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}",args);

    let keyword = &args[1].to_lowercase();
    let name = &args[2];

    println!("Keywords passed in: {}, {}", keyword, name);

    let past_dt = Local::now().date().and_hms(6, 0, 0);
    let dt = Local::now();
    let duration = past_dt.signed_duration_since(dt);
    //println!("Duration: {:?}", duration);
    println!(
        "Time Difference: {:02}:{:02}:{:02}",
        duration.num_hours(),
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    );

    match keyword.as_str() {
        "start" | "init" | "go" => todo!(),
        "stop" | "end" => todo!(),
        _ => println!("Unknown keyword"),
    };
}
