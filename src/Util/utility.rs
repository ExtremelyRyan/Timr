use super::tasks::Task;
use anyhow::{Ok, Result};
use chrono::{Datelike, Duration, NaiveDate, NaiveTime, Utc};
use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::str::FromStr; 

pub const OUTPUT_FILE: &str = "timr.json";

/// getting our starting and ending time for a task, we calculate the difference
/// and return a customized string.
/// # NOTE
/// Currently this only works for same day calculations. this does not take dates into consideration.
pub fn calc_time_diff(start_time: &str, end_time: &str) -> Result<String> {
    let start = NaiveTime::parse_from_str(start_time, "%H%M")?;
    let end = NaiveTime::parse_from_str(end_time, "%H%M")?;

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

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn get_date() -> Result<String> {
    let date = chrono::Local::now();
    Ok(format!("{}-{}-{}", date.year(), date.month(), date.day()))
}


pub fn get_task(task_name: &str, file: Option<&str>) -> Option<Task> {
    let task: Vec<Task> = match file.is_some() {
        true => get_tasks_by_name(task_name.to_string(), file.unwrap()).unwrap(),
        false => get_tasks_by_name(task_name.to_string(), OUTPUT_FILE).unwrap(),
    };
    task.into_iter().find(|t| t.time_end.is_none())
}

pub fn get_tasks_by_name(task_name: String, filename: &str) -> Result<Vec<Task>> {
    
    let tasks = read_all_tasks(filename).unwrap();
    let mut collection: Vec<Task> = Vec::new();

    for t in tasks { 
        if t.task_name == task_name {
            collection.push(t);
        }
    } 

    Ok(collection)
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

pub fn read_all_tasks(filename: &str) -> Result<Vec<Task>> {
    // read data from file
    let collection: Vec<String> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    
    let mut tasks: Vec<Task> = Vec::new();

    for s in collection {
        tasks.push(Task::task_from_string(s));
    }
    Ok(tasks)
}

pub fn read_tasks_from_day_range(days: i32) -> Vec<Task> {
    let mut rtn: Vec<Task> = Vec::new();
    let today = Task::new_task_today();

    // reading all the tasks from the file will get problematic, so this is temporary.
    let raw = read_all_tasks(OUTPUT_FILE).unwrap();
    for s in raw.into_iter() {
        if compare_dates(&s, &today) <= days {
            rtn.push(s);
        }
    }
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
        let task_date = NaiveDate::from_str(&t.date).unwrap();
        // if task date is monday or later:
        match (task_date - monday) >= Duration::zero() {
            true => {
                // add it to this week's tasks
                tasks.push(t);
            }
            false => (),
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

pub fn update_task_in_file(task: Task, file: &str) -> Result<()> {
    let mut collection = read_all_tasks(file).unwrap(); 
    let mut index = 0;

    for (i, t) in collection.iter().enumerate() {
        match t.task_name == task.task_name && t.time_start == task.time_start && t.date == task.date {
            true => {
                index = i;
                break;
                // t.clone_from(&&task);
                // dbg!(&t); 
            }
            false => (),
        } 
    }  

    // TODO:: need to calculate final time and append it to our task to ensure it gets saved to file.

    collection.remove(index);
    collection.insert(index, task);

    let mut buf: Vec<u8> = Vec::new();

    for s in collection { 
        let mut x = s.to_json_string().as_bytes().to_owned().into_iter().collect::<Vec<u8>>();
        buf.append(&mut x); 
    } 

    let mut f = File::create(file)?;
    _ = f.write_all(&buf);  
    Ok(())
}

/**
Compares the converted `NativeDate` date from two Tasks, 
and get the absolute difference of days between the two.

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

pub fn do_test() {
    let t = generate_sample_task();
    let t_json = serde_json::to_string(&t).unwrap();

    _ = output_task_to_file(t); 

    let f = read_all_tasks("timr.json").unwrap();

    let task_to_json = serde_json::to_string(&f[0]).unwrap();

    dbg!(&task_to_json, &t_json);

    assert_eq!(t_json, task_to_json);
}

#[cfg(test)]
mod tests { 

    // required imports for testing
    use super::*;
    use crate::Util::utility;

    // ----------------------------

    #[test]
    pub fn test_calc_time_diff() {
        let start = "0700";
        let end = "1200";
        let res = calc_time_diff(start, end).unwrap();
        assert_eq!(res, "5 hours, 0 minutes".to_string());

        let start = "0700";
        let end = "1900";
        let res = calc_time_diff(start, end).unwrap();
        assert_eq!(res, "12 hours, 0 minutes".to_string());

        let start = "2300";
        let end = "0500";
        let res = calc_time_diff(start, end).unwrap();
        // ! this should be 6 hours, but since date is not a factor we get 18.
        assert_eq!(res, "18 hours, 0 minutes".to_string());
    }

    #[test]
    pub fn test_get_task() {
        let t: Task = Task::new(
            get_date().unwrap(),
            "debugging".to_string(),
            "12:30:00".to_string(),
            None,
            293,
        );
        _ = output_task_to_file(t.clone()); 

        let result = get_task("debugging", Some(OUTPUT_FILE));

        assert_eq!(t, result.unwrap());
 
    }

    #[test]
    pub fn test_output_task_to_file() {
        let t = generate_sample_task();
        let t_json = serde_json::to_string(&t).unwrap();

        _ = output_task_to_file(t); 

        let f = read_all_tasks("timr.json").unwrap();

        let task_to_json = serde_json::to_string(&f[0]).unwrap();

        dbg!(&task_to_json, &t_json);

        assert_eq!(t_json, task_to_json);
    }

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

        let t1: Task = Task::new(
            String::from("date"),
            String::from("task1"),
            String::from("0600"),
            Some(String::from("0800")),
            120,
        );

        let t2: Task = Task::new(
            String::from("date"),
            String::from("task1"),
            String::from("0700"),
            Some(String::from("1000")),
            180,
        );

        assert_eq!(300, utility::sum_task_total_time(t1, t2));
    }

    #[test]
    pub fn test_serde_json() {
        let t: Task = generate_sample_task();
        let json_str = format!("{}\r\n", serde_json::to_string(&t).unwrap());
        _ = prepend_file(json_str.as_bytes(), OUTPUT_FILE);
    }
}
