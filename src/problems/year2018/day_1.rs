use std::collections::HashSet;

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2018/day_1.txt");

pub fn part_1() -> Result<()> {
    let mut sum = 0;

    for data in DATA.lines() {
        sum += data.parse::<i32>()?;
    }

    println!("Part 1: {sum}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut sum = 0;
    let mut record = HashSet::new();

    'outer: loop {
        for data in DATA.lines() {
            sum += data.parse::<i32>()?;
            if !record.insert(sum) {
                break 'outer;
            }
        }
    }

    println!("Part 2: {sum}");
    Ok(())
}
