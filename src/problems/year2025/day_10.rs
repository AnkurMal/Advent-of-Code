use rayon::prelude::*;
use std::collections::HashMap;

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_10.txt");

pub fn part_1() -> Result<()> {
    let mut sum = 0;
    for line in DATA.lines() {
        let spl = line.split_whitespace().collect::<Vec<_>>();
        let switch = spl[0][1..spl[0].len() - 1]
            .chars()
            .map(|x| x == '#')
            .collect::<Vec<_>>();

        let ins = spl[1..spl.len() - 1]
            .iter()
            .map(|x| {
                x[1..x.len() - 1]
                    .split(',')
                    .map(|y| y.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut original = vec![false; switch.len()];
        let mut map = HashMap::new();

        sum += lowest(&switch, &mut original, &ins, &mut map);
    }

    println!("Part 1: {}", sum);
    Ok(())
}

pub fn lowest(
    switch: &[bool],
    original: &mut [bool],
    ins: &[Vec<usize>],
    map: &mut HashMap<Vec<bool>, i32>,
) -> i32 {
    let mut best = i32::MAX;
    if let Some(val) = map.get(original) {
        return *val;
    }
    if original == switch {
        return 0;
    }

    map.insert(original.to_vec(), i32::MAX);
    for i in ins {
        for &j in i {
            original[j] = !original[j];
        }

        let res = lowest(switch, original, ins, map);
        if res != i32::MAX {
            best = best.min(res + 1);
        }

        for &j in i {
            original[j] = !original[j];
        }
    }

    map.insert(original.to_vec(), best);
    best
}
