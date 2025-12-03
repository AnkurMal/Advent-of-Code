use anyhow::Result;
use rayon::prelude::*;

const DATA: &str = include_str!("../../data/year2025/day_2.txt");

pub fn part_1() -> Result<()> {
    let mut counter = 0;

    for i in DATA.split(',') {
        let mut spl = i.split('-');
        let first = spl.next().unwrap().parse::<i64>()?;
        let second = spl.next().unwrap().parse::<i64>()?;

        for j in first..=second {
            let str = j.to_string();
            if str.len().is_multiple_of(2) && str[..str.len() / 2] == str[str.len() / 2..] {
                counter += j;
            }
        }
    }

    println!("Part 1: {counter}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let counter: i64 = DATA
        .split(',')
        .par_bridge()
        .map(|i| {
            let mut spl = i.split('-');
            let first = spl.next().unwrap().parse::<i64>().unwrap();
            let second = spl.next().unwrap().parse::<i64>().unwrap();

            let mut local_counter = 0;

            for j in first..=second {
                let str = j.to_string();

                for k in 1..=str.len() / 2 {
                    let mut vec = vec![];

                    for chunk in (0..str.len()).step_by(k) {
                        if chunk + k < str.len() {
                            vec.push(&str[chunk..chunk + k]);
                        } else {
                            vec.push(&str[chunk..]);
                        }
                    }

                    let el = vec[0];
                    if vec.iter().all(|&x| x == el) {
                        local_counter += j;
                        break;
                    }
                }
            }

            local_counter
        })
        .sum();

    println!("Part 2: {counter}");
    Ok(())
}
