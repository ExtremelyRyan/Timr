#![allow(dead_code)]

use anyhow::Ok;
use chrono::{self, NaiveTime};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
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

pub fn do_parse() -> anyhow::Result<()> {
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
pub fn calc_time_diff(start_time: &str, end_time: &str, cli: &Cli) -> anyhow::Result<String> {
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
pub fn get_current_time() -> anyhow::Result<String> {
    Ok(chrono::Local::now().time().format("%H%M").to_string())
}
