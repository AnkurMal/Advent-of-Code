mod problems;

use anyhow::Result;
#[allow(unused)]
use problems::year2021::*;

fn main() -> Result<()> {
    day_4::part_2()?;

    Ok(())
}
