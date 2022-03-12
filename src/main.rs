use std::path::PathBuf;
use chrono::{self};

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
    }
    
}

fn main() {
    let cli = Cli::parse(); 

    // // You can check the value provided by positional arguments, or option arguments
    // if let Some(name) = cli.name.as_deref() {
    //     println!("Value for name: {name}");
    // }

    // if let Some(time) = cli.time {
    //     println!("time entered: {}", time);
    // }
    // else {
    //     println!("task {} started at: {}", cli.name.as_deref().unwrap() , chrono::offset::Local::now().time().format("%H:%M")); 
    // }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        false => println!("Debug mode is off"),
        true => println!("Debug mode is on"), 
    }

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
            
        }
        Some(Commands::End { task  }) => {
            println!("{} ended at: {}", task , chrono::offset::Local::now().time().format("%H:%M"));
        }
        
        None => {}
    }

    // Continued program logic goes here...
}