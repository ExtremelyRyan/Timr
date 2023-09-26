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
        /// optional end time of task (HHMM)
        #[arg(required = false)]
        time: String,
    },

    /// get a list of tasks
    List {
        /// get a list of all tasks from this week
        #[arg(short, long)]
        week: bool, // TODO

        /// get list of today's tasks
        #[arg(short, long)]
        today: bool, // TODO

        /// get list based on (this years) date string (MMDD)
        #[arg(short, long, required = false)]
        range: String, // TODO

        /// get list based on number of days
        #[arg(short, long, required = false)]
        days: u32, // TODO
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
        Some(Commands::End { task, time }) => {
            // get last task matching that does not have a end time.
            
            match time.is_empty() {
                true => {
                    println!(
                        "{} ended at: {}",
                        task,
                        chrono::offset::Local::now().time().format("%H:%M")
                    );
                }
                false => todo!(),
            };
        }

        Some(Commands::List {
            week,
            today,
            range,
            days,
        }) => {
            dbg!(week, today, range, days);
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
