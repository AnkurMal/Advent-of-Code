use std::collections::{BTreeMap, HashSet};

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_8.txt");

#[derive(Debug, Clone, Copy)]
struct Point(i64, i64, i64);

impl Point {
    fn dist(&self, other: &Point) -> i64 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point(nums[0], nums[1], nums[2])
        })
        .collect()
}

fn merge(mut sets: Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut changed = true;
    while changed {
        changed = false;
        let mut res: Vec<HashSet<usize>> = Vec::new();

        for set in sets {
            if let Some(existing) = res.iter_mut().find(|a| !a.is_disjoint(&set)) {
                existing.extend(&set);
                changed = true;
            } else {
                res.push(set);
            }
        }

        sets = res;
    }
    sets
}

pub fn part_1() -> Result<()> {
    let points = parse(DATA);

    let mut pairs: Vec<(HashSet<usize>, i64)> = (0..points.len())
        .flat_map(|i| {
            ((i + 1)..points.len()).map({
                {
                    let value = points.clone();
                    move |j| {
                        let mut s = HashSet::new();
                        s.insert(i);
                        s.insert(j);
                        (s, value[i].dist(&value[j]))
                    }
                }
            })
        })
        .collect();

    pairs.sort_unstable_by_key(|(_, d)| *d);
    let top_pairs: Vec<HashSet<usize>> = pairs.into_iter().take(1000).map(|(s, _)| s).collect();
    let merged = merge(top_pairs);

    let mut sizes: Vec<usize> = merged.iter().map(|s| s.len()).collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    println!("Part 1: {}", sizes[0] * sizes[1] * sizes[2]);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let points = parse(DATA);

    let mut pairs: Vec<(usize, usize, i64)> = (0..points.len())
        .flat_map(|i| {
            ((i + 1)..points.len()).map({
                let value = points.clone();
                move |j| (i, j, value[i].dist(&value[j]))
            })
        })
        .collect();

    pairs.sort_unstable_by_key(|(_, _, d)| *d);

    let mut components: Vec<HashSet<usize>> = (0..points.len())
        .map(|i| {
            let mut s = HashSet::new();
            s.insert(i);
            s
        })
        .collect();

    for (i, j, _) in pairs {
        let mut idx_i = None;
        let mut idx_j = None;

        for (idx, set) in components.iter().enumerate() {
            if set.contains(&i) {
                idx_i = Some(idx);
            }
            if set.contains(&j) {
                idx_j = Some(idx);
            }
        }

        if let (Some(ii), Some(jj)) = (idx_i, idx_j)
            && ii != jj
        {
            let other = components[jj].clone();
            components[ii].extend(other);
            components.remove(jj);

            if components.len() == 1 {
                let x1 = points[i].0;
                let x2 = points[j].0;
                println!("Part 2: {}", x1 * x2);
                return Ok(());
            }
        }
    }

    Ok(())
}
