use crate::Util::{tasks::Task, utility::*};
use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a task
    // #[cmd(short, long)]
    Start {
        /// name of task ("working on code-review #175")
        task: String,
        /// time started (HH:MM format)
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

pub fn do_parse() -> Result<()> {
    let cli = Cli::parse();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Start { task, time }) => match time.is_some() {
            true => {
                let t: Task = Task::new(
                    get_date().unwrap(),
                    task.to_owned(),
                    time.clone().unwrap(),
                    None,
                    0,
                );
                _ = output_task_to_file(t);
                println!("{} started at: {}", task, time.clone().unwrap());
            }
            false => {
                let current_time = chrono::offset::Local::now()
                    .time()
                    .format("%H:%M")
                    .to_string();
                let t: Task = Task::new(
                    get_date().unwrap(),
                    task.to_owned(),
                    current_time.clone(),
                    None,
                    0,
                );
                _ = output_task_to_file(t);
                println!("{} started at: {}", task, current_time);
            }
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
                Some(end) => calc_time_diff(start, end).unwrap(),

                // otherwise we have to fill in the time.
                None => {
                    let time = get_time()?;
                    calc_time_diff(start, time.as_str()).unwrap()
                }
            };
            println!("{}", result);
        }
        //? should we do something if nothing is entered?
        None => {}
    }

    Ok(())
}
