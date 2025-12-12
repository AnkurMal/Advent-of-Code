use std::collections::{HashMap, HashSet};

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_12.txt");

pub fn part_1() -> Result<()> {
    let spl = &DATA.split("\r\n5:").skip(1).collect::<Vec<&str>>()[0][19..];
    let mut count = 0;

    for line in spl.lines() {
        let spl1 = line.split_whitespace().collect::<Vec<&str>>();
        let spl2 = spl1[0].split('x').collect::<Vec<_>>();

        let n1 = spl2[0].parse::<i32>()?;
        let n2 = spl2[1][..spl2[1].len() - 1].parse::<i32>()?;

        let mul = n1 * n2;
        let mut tot = 0;

        for num in spl1.iter().skip(1) {
            tot += num.parse::<i32>()? * 7;
        }

        if mul >= tot {
            count += 1;
        }
    }

    println!("Part 1: {}", count);
    Ok(())
}

pub fn part_2() -> Result<()> {
    Ok(())
}
