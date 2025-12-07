use std::collections::{HashMap, HashSet};

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_7.txt");

pub fn part_1() -> Result<()> {
    let grid = DATA.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut coord: HashSet<(usize, usize)> = HashSet::from([((grid.len() - 1) / 2, 0)]);

    let mut counter = 0;

    for _ in 0..grid.len() {
        let mut local = HashSet::new();

        for set in coord.iter() {
            if grid[set.1][set.0] == b'^' {
                counter += 1;
                local.insert((set.0 - 1, set.1 + 1));
                local.insert((set.0 + 1, set.1 + 1));
            } else {
                local.insert((set.0, set.1 + 1));
            }
        }

        coord = local;
    }

    println!("Part 1: {}", counter);
    Ok(())
}

pub fn part_2() -> Result<()> {
    let grid = DATA.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut coord = HashMap::from([(((grid.len() - 1) / 2, 0), 1)]);

    for y in 0..grid.len() {
        let mut local = HashMap::new();

        for (&(x, y), &count) in coord.iter() {
            if grid[y][x] == b'^' {
                *local.entry((x - 1, y + 1)).or_insert(0) += count;
                *local.entry((x + 1, y + 1)).or_insert(0) += count;
            } else {
                *local.entry((x, y + 1)).or_insert(0) += count;
            }
        }

        coord = local;
    }

    println!("Part 2: {}", coord.values().sum::<usize>());
    Ok(())
}
