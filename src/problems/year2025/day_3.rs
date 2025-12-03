use std::{ops::Index, str::FromStr};

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_3.txt");

pub fn part_1() -> Result<()> {
    println!("Part 1: {}", get_answer(2));
    Ok(())
}

pub fn part_2() -> Result<()> {
    println!("Part 2: {}", get_answer(12));
    Ok(())
}

fn get_answer(digits: usize) -> i64 {
    DATA.lines()
        .map(|line| {
            let mut max = 0;
            let num = line.as_bytes();
            let mut n = 0;
            let mut ind = 0;

            for i in (0..digits).rev() {
                let k = num[ind..].len() - i;
                let m = *num[ind..ind + k].iter().max().unwrap();

                n += (m - b'0') as i64 * 10i64.pow(i as u32);
                max = n.max(max);
                ind += num[ind..].iter().position(|&x| x == m).unwrap() + 1;
            }

            max
        })
        .sum()
}
