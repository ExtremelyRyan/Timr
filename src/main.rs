#![allow(dead_code, non_snake_case)]
mod Util;
use Util::parser;

fn main() -> anyhow::Result<()> {
    _ = parser::do_parse();

    // _ = timr::read_all_tasks_from_file(OUTPUT_FILE);

    // let range = timr::read_tasks_from_day_range(7);
    // dbg!(range);

    // read_tasks_this_week();

    // test_sum_task_total_time();

    // so after doing all this testing, I think total_time is just going to have to be a string containing total minutes,
    // and each time we want to add to it, we will have to parse it out to a i32, manually parse the timestamp, and then add
    // the two together, then re-save as a string.

    // this is a HUGE bummer, since it seems like there is NO way we can add two DateTime, NaiveTime, etc without wonky conversions
    // to Durations (which doesnt really work well anyway).
    // let _current_total = String::from("560").parse::<i32>().unwrap();
    // let timestamp = String::from("3:15");
    // let v: Vec<&str> = timestamp.split(":").collect();
    // let aggregate: i32 = v[0].parse::<i32>().unwrap() * 60 + v[1].parse::<i32>().unwrap();

    // dbg!(aggregate);

    Ok(())
}
