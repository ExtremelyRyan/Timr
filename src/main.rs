#![allow(dead_code, non_snake_case)]

mod Util;

fn main() -> anyhow::Result<()> {
    Util::parser::do_parse()
}
