use anyhow::Ok;
use chrono::format::format;
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
struct Task {
    date: String,
    task_name: String,
    time_start: String,
    time_end: Option<String>,
    time_total: Option<String>,
}

impl Task {
    fn new(
        date: String,
        task_name: String,
        time_start: String,
        time_end: Option<String>,
        time_total: Option<String>,
    ) -> Self {
        Self {
            date,
            task_name,
            time_start,
            time_end,
            time_total,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Start a task
    // #[cmd(short, long)]
    Start {
        /// name of task  
        task: String,
        /// time started
        time: Option<String>,
    },
    /// End a task
    End {
        /// name of task
        #[arg(required = true)]
        task: String,
    },

    /// get difference between two time inputs, seperated by a space
    Calc {
        /// starting time: 24hr format (i.e 1630)
        #[arg(required = true)]
        start_time: String,
        /// optional ending time: 24hr format (i.e 1800)
        #[arg(required = false)]
        ending_time: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    test_serde_json();
    read_from_file("timr.json".to_string());

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Start { task, time }) => match time.is_some() {
            true => println!("{} started at: {}", task, time.as_deref().unwrap()),
            false => println!(
                "{} started at: {}",
                task,
                chrono::offset::Local::now().time().format("%H:%M")
            ),
        },
        Some(Commands::End { task }) => {
            println!(
                "{} ended at: {}",
                task,
                chrono::offset::Local::now().time().format("%H:%M")
            );
        }

        Some(Commands::Calc {
            start_time,
            ending_time,
        }) => {
            let result: String = match ending_time {
                Some(end) => calc_time_diff(start_time, end, &cli).unwrap(),
                None => {
                    let time = get_current_time().unwrap();
                    calc_time_diff(start_time, time.as_str(), &cli).unwrap()
                }
            };
            println!("{}", result);
        }

        None => {}
    }
}

fn calc_time_diff(start_time: &str, end_time: &str, cli: &Cli) -> anyhow::Result<String> {
    // let time_str = "22:10:57";
    let start = NaiveTime::parse_from_str(start_time, "%H%M")?;
    let end = NaiveTime::parse_from_str(end_time, "%H%M")?;

    if cli.debug {
        println!("start: {}\t end {}", start, end);
    }

    let hours = (end - start).num_hours();
    let hours_in_min = hours * 60;
    let minutes = (end - start).num_minutes() - hours_in_min;

    Ok(format!("{} hours, {} minutes", hours.abs(), minutes.abs()))
}

fn get_current_time() -> anyhow::Result<String> {
    // get current time
    let binding = chrono::Local::now().time().format("%H%M").to_string();

    Ok(binding)
}

fn test_serde_json() {
    let t = generate_sample_task();

    let j = serde_json::to_string(&t).unwrap();
    let task_str = format!("{}\r\n", j);

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("timr.json")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write_all(task_str.as_bytes())
        .expect("write failed");
}

fn read_from_file(filename: String) {
    // read data from file
    let v: Vec<String> = std::fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect();  // gather them together into a vector

    let mut t: Vec<Task> = Vec::new();

    // create struct from string
    for lines in v {
        let temp: Task = serde_json::from_str(&lines).unwrap();
        t.push(temp);
    }

    dbg!(t);

}

fn generate_sample_task() -> Task {
    let date = chrono::Local::now();
    let date_s = format!("{}-{}-{}", date.year(), date.month(), date.day());

    let time_start = NaiveTime::from_hms(8, 0, 0);
    let time_end = NaiveTime::from_hms(16, 0, 0);
    let time_total = time_end - time_start;

    // dbg!(date, time_start, time_end);

    Task::new(
        date_s,
        String::from("Sample Task"),
        time_start.to_string(),
        Some(time_end.to_string()),
        Some(time_total.to_string()),
    )
}
