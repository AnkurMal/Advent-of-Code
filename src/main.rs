mod problems;

use anyhow::Result;
#[allow(unused)]
use problems::year2018::*;

fn main() -> Result<()> {
    day_2::part_2()?;

    Ok(())
}
