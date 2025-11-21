use std::collections::{HashMap, hash_map::Entry};

use anyhow::Result;
use num::integer;

const DATA: &str = include_str!("../../data/year2023/day_8.txt");

pub fn part_1() -> Result<()> {
    let mut spl = DATA.split("\r\n\r\n");
    let mut counter = 0;

    let direction = spl.next().unwrap();
    let mut node = HashMap::new();
    let mut elem = "AAA";

    for i in spl.next().unwrap().lines() {
        node.insert(&i[..3], (&i[7..10], &i[12..15]));
    }

    'outer: loop {
        for dir in direction.chars() {
            if dir == 'R' {
                elem = node[elem].1;
            } else {
                elem = node[elem].0;
            }

            counter += 1;
            if elem == "ZZZ" {
                break 'outer;
            }
        }
    }

    println!("Part 1: {counter}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut spl = DATA.split("\r\n\r\n");

    let direction = spl.next().unwrap();
    let mut node = HashMap::new();
    let mut lcm = HashMap::new();
    let mut elem = Vec::new();

    for i in spl.next().unwrap().lines() {
        node.insert(&i[..3], (&i[7..10], &i[12..15]));

        if i[..3].ends_with('A') {
            elem.push((&i[..3], 0));
        }
    }

    'outer: loop {
        for dir in direction.chars() {
            for (ind, (el, counter)) in elem.iter_mut().enumerate() {
                if let Entry::Vacant(e) = lcm.entry(ind) {
                    if dir == 'R' {
                        *el = node[el].1;
                    } else {
                        *el = node[el].0;
                    }

                    *counter += 1;
                    if el.ends_with('Z') {
                        e.insert(*counter);
                    }
                }
            }

            if lcm.len() == elem.len() {
                break 'outer;
            }
        }
    }

    let mut ans: i64 = 1;
    for val in lcm.values() {
        ans = integer::lcm(ans, *val);
    }

    println!("Part 2: {ans}");
    Ok(())
}
