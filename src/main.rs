
use std::{path::PathBuf, time::SystemTime};
use chrono::{NaiveTime, Utc, Local};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli { 
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on 
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values 
        list: bool,
    },
    /// Start a task
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
        /// starting time 
        #[arg(required = true)]
        start_time: String,
        /// ending time 
        #[arg(required = false)]
        ending_time: Option<String>,
    }
    
}

fn calc_time_diff(start_time: &str, end_time: &str) -> Result<String, anyhow::Error> { 
    // let time_str = "22:10:57"; 
    let start = NaiveTime::parse_from_str(start_time, "%H%M")?;
    let end   = NaiveTime::parse_from_str(end_time  , "%H%M").unwrap_or(NaiveTime::from(Utc::now().time()));

    dbg!(start, end); 

    let hours = (end - start).num_hours();
    let hours_in_min = hours * 60;
    let minutes = (end - start).num_minutes() - hours_in_min; 


    Ok(format!("{} hours, {} minutes", hours.abs(), minutes.abs()))
}

fn main() {
    // let time_diff = testing("0645", "1600");
    // if let Ok(td) = time_diff { println!("{td}"); }
    

    let cli = Cli::parse(); 

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     false => println!("Debug mode is off"),
    //     true => println!("Debug mode is on"), 
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        },
        Some(Commands::Start { task, time }) => {
            match time.is_some() {
                true  => println!("{} started at: {}", task , time.as_deref().unwrap()),
                false => println!("{} started at: {}", task , chrono::offset::Local::now().time().format("%H:%M")),
            }
            
        },
        Some(Commands::End { task  }) => {
            println!("{} ended at: {}", task , chrono::offset::Local::now().time().format("%H:%M"));
        },

        Some(Commands::Calc { start_time, ending_time }) => {

            let mut result: String = String::from("");
            match ending_time {
                Some(end) =>  result = calc_time_diff(start_time, end).unwrap(),
                None => result = calc_time_diff(start_time, &format!("{:?}",SystemTime::now()).to_string() ).unwrap(),
            } 
            println!("{}", result);
        }

        
        None => {}
    }

    // Continued program logic goes here...
}