#![warn(dead_code)]

use anyhow::Ok;
use chrono::{NaiveDate, NaiveTime};
use chrono::{Datelike, Duration, Utc};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;
use crate::task::Task;

mod parse;
mod task;

const OUTPUT_FILE: &str = "timr.json";

/// testing our sample task generation, converting json string to task,
pub fn test_serde_json() {
    let t = task::generate_sample_task();

    let json_str = serde_json::to_string(&t).unwrap();
    let json_str = format!("{}\r\n", json_str);
    _ = prepend_file(json_str.as_bytes(), &OUTPUT_FILE);
}

/// simple prepending file
pub fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> anyhow::Result<()> {
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

pub fn sum_task_total_time(t1: Task,t2: Task) -> Duration {
    if t1.task_name != t2.task_name { return Duration::zero(); }

    let mut hour = String::new();
    let mut min= String::new();

    let time_parsed: Vec<String> = t1.time_total.unwrap()
    .split_ascii_whitespace()
    .into_iter()
    .filter_map(|s| s.trim().parse().ok())
    .collect();
dbg!(&time_parsed);
    hour = match time_parsed[0].len() == 1 {
        true  => format!("0{}",time_parsed[0].to_string() ),
        false => format!("{}",time_parsed[0].to_string() ),
    };
    min = match time_parsed[1].len() == 1 {
        true  => format!("0{}",time_parsed[2].to_string()), 
        false => format!("{}",time_parsed[2].to_string()),
    };
    
    let time1 = format!("{}:{}", hour, min);

    let mut hour = String::new();
    let mut min= String::new();

    let time_parsed: Vec<String> = t2.time_total.unwrap()
    .split_ascii_whitespace()
    .into_iter()
    .filter_map(|s| s.trim().parse().ok())
    .collect();
    hour = match time_parsed[0].len() == 1 {
        true  => format!("{}",time_parsed[0].to_string() ),
        false => format!("{}",time_parsed[0].to_string() ),
    };
    min = match time_parsed[1].len() == 1 {
        true  => format!("{}",time_parsed[2].to_string()), 
        false => format!("{}",time_parsed[2].to_string()),
    };

    let time2 = format!("{}:{}", hour, min);

    dbg!(&time1, &time2);

    let time_conv1 = chrono::NaiveTime::parse_from_str(&time1, "%H:%M").unwrap();
    let time_conv2 = chrono::NaiveTime::parse_from_str(&time2, "%H:%M").unwrap();

    (time_conv1 - time_conv2)
}

pub fn test_sum_task_total_time() {
    let task1 = Task::new(
        "2023-9-7".to_string(),
     "test".to_string(),
      "0800".to_string(),
      Some("1200".to_string()),
      Some("4 hours 0 minutes".to_string()),
    );
    let task2 = Task::new(
        "2023-9-8".to_string(),
     "test".to_string(),
      "0800".to_string(),
      Some("1200".to_string()),
      Some("4 hours 0 minutes".to_string()),
    );

    let result = sum_task_total_time(task1, task2);
    dbg!(result);
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
    use crate::*;

    #[test]
    fn test_compare_dates() {
        let t1: Task = Task::new(
            "2023-9-1".to_string(),
            "debugging".to_string(),
            "11:07:32".to_string(),
            Some("16:00:53".to_string()),
            Some("4 hours 53 minutes".to_string()),
        );
        let t2: Task = Task::new(
            "2023-9-7".to_string(),
            "debugging".to_string(),
            "11:07:32".to_string(),
            Some("16:00:53".to_string()),
            Some("4 hours 53 minutes".to_string()),
        );

        let comparison = compare_dates(&t2, &t1);
        assert_eq!(comparison, 6);
    }
}
