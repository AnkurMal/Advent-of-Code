use std::collections::HashMap;

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_11.txt");

pub fn part_1() -> Result<()> {
    let mut vec = DATA
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    vec.iter_mut().for_each(|x| x[0] = &x[0][..x[0].len() - 1]);

    println!("Part 1: {}", find_out("you", &vec));
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut vec = DATA
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    vec.iter_mut().for_each(|x| x[0] = &x[0][..x[0].len() - 1]);
    let mut map = HashMap::new();

    println!("Part 2: {}", count_out("svr", &vec, 0, &mut map));
    Ok(())
}

fn count_out<'a>(
    st: &'a str,
    vec: &'a [Vec<&str>],
    store: i64,
    memo: &mut HashMap<(&'a str, i64), i64>,
) -> i64 {
    if let Some(&count) = memo.get(&(st, store)) {
        return count;
    }

    if st == "out" {
        return if store == 2 { 1 } else { 0 };
    }

    let mut count = 0;
    for i in vec {
        if i[0] == st {
            for &j in i.iter().skip(1) {
                let new_store = if j == "fft" || j == "dac" {
                    store + 1
                } else {
                    store
                };
                count += count_out(j, vec, new_store, memo);
            }
        }
    }

    memo.insert((st, store), count);
    count
}

fn find_out(st: &str, vec: &[Vec<&str>]) -> i64 {
    if st == "out" {
        return 1;
    }

    let mut count = 0;
    for i in vec {
        if i[0] == st {
            for &j in i.iter().skip(1) {
                count += find_out(j, vec);
            }
        }
    }

    count
}
