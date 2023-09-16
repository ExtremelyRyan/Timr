use anyhow::{Ok, Result};
use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Utc};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use super::{parser::Cli, tasks::Task};

const OUTPUT_FILE: &str = "./timr.json";

/// getting our starting and ending time for a task, we calculate the difference
/// and return a customized string.
pub fn calc_time_diff(start_time: &str, end_time: &str, cli: &Cli) -> Result<String> {
    let start = NaiveTime::parse_from_str(start_time, "%H%M")?;
    let end = NaiveTime::parse_from_str(end_time, "%H%M")?;

    if cli.debug {
        println!("start: {}\t end {}", start, end);
    }

    let hours = (end - start).num_hours();
    let hours_in_min = hours * 60;
    let minutes = (end - start).num_minutes() - hours_in_min;
    match hours {
        0 => Ok(format!("{} minutes", minutes.abs())),
        _ => Ok(format!("{} hours, {} minutes", hours.abs(), minutes.abs())),
    } 
}

pub fn output_task_to_file(t: Task) -> Result<()> { 
    let fstr = format!("{}\r\n", serde_json::to_string(&t).unwrap());
    _ = prepend_file(fstr.as_bytes(), OUTPUT_FILE);
    Ok(())
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
    // let time_total = format!("{} hours {} minutes", hours, minutes);

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
        hours_in_min + minutes,
    )
}

/// I am tired of having to remember how to get a formatted version of the current time,
/// so now we have this function.
/// # Return
/// * string is formatted into the `%H%M`, seconds are not included.
/// # Examples
/// * "0645"
pub fn get_time() -> Result<String> {
    Ok(chrono::Local::now().time().format("%H%M").to_string())
}

pub fn get_date() -> Result<String> {
    let date = chrono::Local::now();
    Ok(format!("{}-{}-{}", date.year(), date.month(), date.day()))
}

/// simple prepending file
pub fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> Result<()> {
    let mut f = File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}

pub fn read_all_tasks(filename: &str) -> anyhow::Result<Vec<String>> {
    // read data from file
    Ok(std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect())
}

pub fn read_tasks_from_day_range(days: i32) -> Vec<Task> {
    let mut rtn: Vec<Task> = Vec::new();
    let today = Task::new_task_today();

    // reading all the tasks from the file will get problematic, so this is temporary.
    let raw = read_all_tasks(OUTPUT_FILE).unwrap();
    raw.into_iter().for_each(|s| {
        if !s.is_empty() {
            let temp: Task = serde_json::from_str(&s).unwrap();
            if compare_dates(&temp, &today) <= days {
                rtn.push(temp);
            }
        }
    });

    rtn
}

pub fn read_tasks_this_week() -> Vec<Task> {
    let monday = NaiveDate::from_isoywd_opt(
        Utc::now().year(),
        Utc::now().iso_week().week(),
        chrono::Weekday::Mon,
    )
    .unwrap();

    let mut tasks: Vec<Task> = Vec::new();

    // reading all the tasks from the file will get problematic, so this is temporary.
    let raw = read_all_tasks(OUTPUT_FILE).unwrap();

    for t in raw {
        if !t.is_empty() {
            let task: Task = serde_json::from_str(&t).unwrap();
            let task_date = NaiveDate::from_str(&task.date).unwrap();
            // if task date is monday or later:
            match (task_date - monday) >= Duration::zero() {
                true => {
                    // add it to this week's tasks
                    tasks.push(task);
                }
                false => (),
            }
        }
    }
    dbg!(&tasks);
    tasks
}

pub fn sum_task_total_time(t1: Task, t2: Task) -> i64 {
    if t1.task_name != t2.task_name {
        return -1;
    }
    t1.time_total + t2.time_total
}

/**
Compares the converted `NativeDate` date from two Tasks, and get the absolute difference of days between the two.

Returns a -1 if either task is missing a `NativeDate`.
# Example
```no_run
let t1: Task = Task::new("2023-9-1".to_string(), ..Default::default() );
let t2: Task = Task::new("2023-9-7".to_string(), ..Default::default() );
assert_eq!(compare_dates(t2, t1), 6);
```
*/
pub fn compare_dates(t1: &Task, t2: &Task) -> i32 {
    if t1.date.is_empty() || t2.date.is_empty() {
        return -1;
    }

    let t1_date = NaiveDate::parse_from_str(&t1.date, "%Y-%m-%d").unwrap();
    let t2_date = NaiveDate::parse_from_str(&t2.date, "%Y-%m-%d").unwrap();

    i64::abs((t1_date - t2_date).num_days()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Util::utility::compare_dates;

    #[test]
    fn test_compare_dates() {
        let t1: Task = Task::new(
            "2023-9-1".to_string(),
            "debugging".to_string(),
            "11:07:32".to_string(),
            Some("16:00:53".to_string()),
            293,
        );
        let t2: Task = Task::new(
            "2023-9-7".to_string(),
            "debugging".to_string(),
            "11:07:32".to_string(),
            Some("16:00:53".to_string()),
            293,
        );

        let comparison = compare_dates(&t2, &t1);
        assert_eq!(comparison, 6);
    }

    #[test]
    pub fn test_sum_task_total_time() {
        let task1 = Task::new(
            "2023-9-7".to_string(),
            "test".to_string(),
            "0800".to_string(),
            Some("1200".to_string()),
            240,
        );
        let task2 = Task::new(
            "2023-9-8".to_string(),
            "test".to_string(),
            "0800".to_string(),
            Some("1200".to_string()),
            240,
        );

        let result = sum_task_total_time(task1, task2);
        assert_eq!(result, 480);
    }

    #[test]
    pub fn test_serde_json() {
        let t: Task = generate_sample_task();
        let json_str = format!("{}\r\n", serde_json::to_string(&t).unwrap());
        _ = prepend_file(json_str.as_bytes(), OUTPUT_FILE);
    }
}
