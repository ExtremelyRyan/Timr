use chrono::{Datelike, NaiveDate, NaiveTime};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Default)]
pub struct Task {
    pub date: String,
    pub task_name: String,
    pub time_start: String,
    pub time_end: Option<String>,
    pub time_total: Option<String>,
}

impl Task {
    pub fn new(
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

    pub fn new_task_today() -> Self {
        // create a temp task for today
        let date = chrono::Local::now();
        let date_s = format!("{}-{}-{}", date.year(), date.month(), date.day());
        Self {
            date: date_s,
            ..Default::default()
        }
    }
    pub fn new_task_from_date(date: NaiveDate) -> Self {
        // create a temp task for a specified date.
        Self {
            date: date.to_string(),
            ..Default::default()
        }
    }
}

pub fn generate_sample_task() -> Task {
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
