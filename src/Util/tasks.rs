use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize, Default, Clone)]
pub struct Task {
    pub date: String,
    pub task_name: String,
    pub time_start: String,
    pub time_end: Option<String>,
    pub time_total: i64,
}

impl Task {
    pub fn new(
        date: String,
        task_name: String,
        time_start: String,
        time_end: Option<String>,
        time_total: i64,
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
    pub fn task_from_string(s: String) -> Self { 
        let t: Task = match serde_json::from_str(s.as_str()) {
            Ok(task) => task,
            Err(e) => panic!("error parsing task from json: {e}"),
        };
        Self {
            date: t.date,
            task_name: t.task_name,
            time_start: t.time_start,
            time_end: t.time_end,
            time_total: t.time_total,
        }
    } 

    pub fn to_json_string(self) -> String {
        let mut s = serde_json::to_string(&self).unwrap();
        s.insert(s.len(), '\n');
        return s;
    }
}
