#![allow(dead_code)]

use crate::task::Task;
use anyhow::Ok;
use chrono::NaiveDate;
use chrono::{Datelike, Duration, Utc};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr;

mod parse;
mod task;

const OUTPUT_FILE: &str = "timr.json";

/// testing our sample task generation, converting json string to task,

/// simple prepending file
pub fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> anyhow::Result<()> {
    let mut f = File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}



pub fn read_tasks_from_day_range(days: i32) -> Vec<Task> {
    let mut rtn: Vec<Task> = Vec::new();
    let today = Task::new_task_today();

    // reading all the tasks from the file will get problematic, so this is temporary.
    let raw = task::read_all_tasks(OUTPUT_FILE).unwrap();
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
    let raw = task::read_all_tasks(OUTPUT_FILE).unwrap();

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
    let task1_time_parsed: i64 = match t1.time_total {
        Some(s) => s.parse().unwrap(),
        None => 0,
    };
    let task2_time_parsed: i64 = match t2.time_total {
        Some(s) => s.parse().unwrap(),
        None => 0,
    };

    task1_time_parsed + task2_time_parsed
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

    #[test]
    pub fn test_sum_task_total_time() {
        let task1 = Task::new(
            "2023-9-7".to_string(),
            "test".to_string(),
            "0800".to_string(),
            Some("1200".to_string()),
            Some("240".to_string()),
        );
        let task2 = Task::new(
            "2023-9-8".to_string(),
            "test".to_string(),
            "0800".to_string(),
            Some("1200".to_string()),
            Some("240".to_string()),
        );
    
        let result = sum_task_total_time(task1, task2);
        assert_eq!(result, 480);
    }

    #[test]
    pub fn test_serde_json() {
        let t = task::generate_sample_task();
    
        let json_str = serde_json::to_string(&t).unwrap();
        let json_str = format!("{}\r\n", json_str);
        _ = prepend_file(json_str.as_bytes(), &OUTPUT_FILE);
    }
}
