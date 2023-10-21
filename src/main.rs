mod util;

fn main() -> anyhow::Result<()> {
    util::parser::do_parse()
}