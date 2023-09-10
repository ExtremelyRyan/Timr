use anyhow::Ok;
use chrono::{Datelike, NaiveTime};
use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

const OUTPUT_FILE: &str = "timr.json";

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
        start: String,
        /// optional ending time: 24hr format (i.e 1800)
        #[arg(required = false)]
        end: Option<String>,
    },
}

fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();
    test_serde_json();
    _ = read_all_tasks_from_file(OUTPUT_FILE.to_string());

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

        Some(Commands::Calc { start, end }) => {
            let result: String = match end {
                
                // if user has entered a ending time, we process like normal.
                Some(end) => calc_time_diff(start, end, &cli).unwrap(),
                
                // otherwise we have to fill in the time.
                None => {
                    let time = get_current_time()?;
                    calc_time_diff(start, time.as_str(), &cli).unwrap()
                }
            };
            println!("{}", result);
        }
        //? should we do something if nothing is entered?
        None => {}
    }
    
    Ok(())
}

/// getting our starting and ending time for a task, we calculate the difference
/// and return a customized string.
fn calc_time_diff(start_time: &str, end_time: &str, cli: &Cli) -> anyhow::Result<String> {
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

/// I am tired of having to remember how to get a formatted version of the current time,
/// so now we have this function.
/// # Return
/// * string is formatted into the `%H%M`, seconds are not included.
/// # Examples
/// * "0645"
fn get_current_time() -> anyhow::Result<String> { 
    Ok(chrono::Local::now().time().format("%H%M").to_string())
}


fn test_serde_json() {
    let t = generate_sample_task();

    let j = serde_json::to_string(&t).unwrap();
    let task_str = format!("{}\r\n", j);

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(OUTPUT_FILE)
        .expect("cannot open file");

    // Write to a file
    data_file
        .write_all(task_str.as_bytes())
        .expect("write failed");
}

fn read_all_tasks_from_file(filename: String) -> anyhow::Result<()> {
    // read data from file
    let collection: Vec<String> = std::fs::read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect(); // gather them together into a vector

    let mut t: Vec<Task> = Vec::new();

    // create struct from string
    for lines in collection {
        let temp: Task = serde_json::from_str(&lines).unwrap();
        t.push(temp);
    }

    dbg!(t);

    Ok(())
}

fn generate_sample_task() -> Task {
    let date = chrono::Local::now();
    let date_s = format!("{}-{}-{}", date.year(), date.month(), date.day());

    let mut rng = rand::thread_rng();

    // generate starting time (between 5-am and 9am)
    let time_start = NaiveTime::from_hms_opt(
        rng.gen_range(5..12),
        rng.gen_range(0..59),
        rng.gen_range(0..59),
    )
    .unwrap();

    // generate ending time (between 2pm and 7pm)
    let time_end = NaiveTime::from_hms_opt(
        rng.gen_range(13..18),
        rng.gen_range(0..59),
        rng.gen_range(0..59),
    )
    .unwrap();

    // hand-roll total time difference.
    let hours = (time_end - time_start).num_hours();
    let hours_in_min = hours * 60;
    let minutes = (time_end - time_start).num_minutes() - hours_in_min;
    // todo: will likely need to come back to reformat how our time_total looks.
    let time_total = format!("{} hours {} minutes", hours, minutes);

    let tasks = [
        "sleeping",
        "refactoring",
        "writing software",
        "fixing bugs",
        "creating new feature",
        "debugging",
        "stuck, pls help",
        "getting coffee",
    ];

    let random_task: String = tasks
        .get(rng.gen_range(0..tasks.len()))
        .unwrap()
        .to_string();

    Task::new(
        date_s,
        random_task,
        time_start.to_string(),
        Some(time_end.to_string()),
        Some(time_total.to_string()),
    )
}
