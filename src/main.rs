use chrono::NaiveTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;

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

fn calc_time_diff(start_time: &str, end_time: &str, cli: &Cli) -> Result<String, anyhow::Error> {
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
    let t: Task = Task::new(date, task_name, time_start, time_end, time_total){}
}
