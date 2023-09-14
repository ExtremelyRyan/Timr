use anyhow::Ok;
use chrono::{DateTime, NaiveTime, NaiveDate};
use std::fs::{OpenOptions, File};
use std::io::{Write, Seek, Read};
use std::path::Path;
use timr::parse::Task;

mod parse;

const OUTPUT_FILE: &str = "timr.json";

fn main() -> anyhow::Result<()> {
    _ = parse::do_parse();

    test_serde_json();

    _ = read_all_tasks_from_file(OUTPUT_FILE);

    Ok(())
}

fn test_serde_json() {
    let t = timr::generate_sample_task();

    let j = serde_json::to_string(&t).unwrap();
    let task_str = format!("{}\r\n", j);

    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(OUTPUT_FILE)
        .expect("cannot open file");

    // Write to a file
    // data_file
    //     .write_all(task_str.as_bytes())
    //     .expect("write failed");
    _ = prepend_file(task_str.as_bytes(), &OUTPUT_FILE);
}

/// simple prepending file
fn prepend_file<P: AsRef<Path> + ?Sized>(data: &[u8], path: &P) -> anyhow::Result<()> {
    let mut f =  File::open(path)?;
    let mut content = data.to_owned();
    f.read_to_end(&mut content)?;

    let mut f = File::create(path)?;
    f.write_all(content.as_slice())?;

    Ok(())
}

fn read_all_tasks_from_file(filename: &str) -> anyhow::Result<Vec<String>> {
    // read data from file
    let collection: Vec<String> = std::fs::read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect(); // gather them together into a vector

    // let mut t: Vec<Task> = Vec::new();

    // // create struct from string
    // for lines in collection {
    //     let temp: Task = serde_json::from_str(&lines).unwrap();
    //     t.push(temp);
    // }

    // dbg!(t);

    Ok(collection)
}

pub fn read_tasks_from_day_range(days: u8) -> Vec<Task> {
    let rtn: Vec<Task> = Vec::new();

    let raw = read_all_tasks_from_file(OUTPUT_FILE).unwrap();
    for s in raw {
        let temp: Task = serde_json::from_str(&s).unwrap();
        // if temp::date {}
        // rtn.push(temp);
    }

    if days == 0 {
        // return todays tasks
    }

    rtn
}

pub fn compare_dates(t1: Task, t2: Task) -> i64 {
    let t1_date = NaiveDate::parse_from_str(&t1.date, "%Y-%m-%d").unwrap();
    let t2_date = NaiveDate::parse_from_str(&t2.date, "%Y-%m-%d").unwrap();

    i64::abs((t1_date - t2_date).num_days())
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

        let comparison = compare_dates(t2, t1);
        assert_eq!(comparison, 6);
    }
}
