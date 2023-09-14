#![warn(dead_code)]

use timr::read_tasks_this_week;

mod parse;
mod task;

const OUTPUT_FILE: &str = "timr.json";

fn main() -> anyhow::Result<()> {
    // _ = parse::do_parse();

    // timr::test_serde_json();

    // _ = timr::read_all_tasks_from_file(OUTPUT_FILE);

    // let range = timr::read_tasks_from_day_range(7);
    // dbg!(range);

    read_tasks_this_week();

    Ok(())
}
