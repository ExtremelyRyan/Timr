use crate::util::{tasks::Task, utility::*};
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
        time: Option<String>,
    },

    /// get a list of tasks. List with no arguments returns a list of unended tasks
    List {
        /// get a list of all tasks from this week
        #[arg(short, long, required = false)]
        week: bool,

        /// get list of today's tasks
        #[arg(short, long, required = false)]
        today: bool,

        /// get list based on number of days
        #[arg(short, long, required = false)]
        days: Option<i32>,
    },

    Fix {
        /// name of task
        #[arg(required = true)]
        task: String,

        /// days to search for task
        #[arg(required = false)]
        days: i64,

        /// amend start time (HH:MM format)
        #[arg(short, long, required = false)]
        start: Option<String>,

        /// amend end time (HH:MM format)
        #[arg(short, long, required = false)]
        end: Option<String>,
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
        Some(Commands::Start { task, time }) => {
            if check_if_task_exists(task.clone()) {
                println!("\nthere is already a incomplete task with that name. \ndo you wish to create a new task? Y/N");
                let mut resp = String::new();
                std::io::stdin().read_line(&mut resp).unwrap();
                match resp.as_str().trim() {
                    "y" | "yes" => (),
                    "n" | "no" => {
                        println!("task canceled.");
                        std::process::exit(0);
                    }
                    _ => {
                        println!("invalid input, cancelling new task.");
                        std::process::exit(0);
                    }
                }
            }
            match time.is_some() {
                true => {
                    let t: Task =
                        Task::new(get_date()?, task.to_owned(), time.clone().unwrap(), None, 0);
                    _ = output_task_to_file(t);
                    println!("{} started at: {}", task, time.clone().unwrap());
                }
                false => {
                    let current_time = chrono::offset::Local::now()
                        .time()
                        .format("%H%M")
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
            }
        }
        Some(Commands::End { task, time }) => {
            // get last task matching that does not have a end time.
            let mut t: Task = get_task(task, Some(OUTPUT_FILE), false).unwrap();
            // dbg!(&task, &time);
            match time.is_some() {
                true => {
                    let ending = time.clone().unwrap();
                    t.time_end = Some(ending);
                }
                false => {
                    let end = chrono::offset::Local::now()
                        .time()
                        .format("%H%M")
                        .to_string();
                    t.time_end = Some(end);
                }
            };
            println!("{} ended at: {}", task, t.clone().time_end.unwrap());
            _ = update_task_in_file(t, OUTPUT_FILE);
        }

        Some(Commands::Fix {
            task,
            days,
            start,
            end,
        }) => {
            if *days >= 1 {
                let tasks: Vec<Task> = read_tasks_from_day_range(*days as i32);
                if tasks.len() > 1 {
                    println!("please choose which task named {task} to modify:");
                }
                let count = 1;
                for t in tasks {
                    println!(
                        "{}. {} \t {} \t {}",
                        count, t.task_name, t.date, t.time_start
                    );
                }
                // get index from user
                let mut resp = String::new();
                std::io::stdin().read_line(&mut resp).unwrap();

                let index = resp.trim().parse::<i32>().unwrap();
            }
        }

        Some(Commands::List { week, today, days }) => {
            if today.clone() {
                let tasks = read_tasks_from_day_range(0);
                for t in tasks {
                    println!("{}", t.print().unwrap());
                }
            }

            if week.clone() {
                let tasks = read_tasks_this_week();
                for t in tasks {
                    println!("{}", t.print().unwrap());
                }
            }

            if days.is_some() {
                let tasks = read_tasks_from_day_range(days.unwrap());
                for t in tasks {
                    println!("{}", t.print().unwrap());
                }
            }

            if !today && !week && days.is_none() {
                let tasks = read_incomplete_tasks();
                for t in tasks {
                    println!("{}", t.print().unwrap());
                }
            }
        }

        Some(Commands::Calc { start, end }) => {
            let result: String = match end {
                // if user has entered a ending time, we process like normal.
                Some(end) => calc_time_diff(start, end).0.to_string(),

                // otherwise we have to fill in the time.
                None => {
                    let time = get_time()?;
                    calc_time_diff(start, time.as_str()).0.to_string()
                }
            };
            let (hour, min) = result.split_at(2);
            println!("{} hours and {} minutes", hour, min);
        }
        //? should we do something if nothing is entered?
        None => {}
    }

    Ok(())
}
