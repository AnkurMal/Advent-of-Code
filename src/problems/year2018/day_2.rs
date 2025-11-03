use std::collections::HashMap;

use anyhow::{Ok, Result};

const DATA: &str = include_str!("../../data/year2018/day_2.txt");

pub fn part_1() -> Result<()> {
    let (mut a, mut b) = (0, 0);

    for data in DATA.lines() {
        let mut map = HashMap::new();
        for ch in data.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        let (mut two, mut three) = (0, 0);
        for val in map.values() {
            if *val == 2 {
                two = 1;
            } else if *val == 3 {
                three = 1;
            }
        }

        a += two;
        b += three;
    }

    println!("Part 1: {}", a * b);
    Ok(())
}

pub fn part_2() -> Result<()> {
    'outer: for (i, st1) in DATA.lines().enumerate() {
        'inner: for st2 in DATA.lines().skip(i + 1) {
            let mut ind = 0;
            let mut found = false;

            for ((j, (ch1, ch2))) in st1.chars().zip(st2.chars()).enumerate() {
                if ch1 != ch2 {
                    if !found {
                        ind = j;
                        found = true;
                    } else {
                        continue 'inner;
                    }
                }
            }

            if found {
                let mut new = st1.to_string();
                new.remove(ind);
                println!("Part 2: {new}");
                break 'outer;
            }
        }
    }

    Ok(())
}
