use std::collections::HashSet;

use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_5.txt");

pub fn part_1() -> Result<()> {
    let spl = DATA.split("\r\n\r\n").collect::<Vec<_>>();
    let mut counter = 0;

    let mut range = vec![];
    for l in spl[0].lines() {
        range.push(
            l.split('-')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    for i in spl[1].lines() {
        let id = i.parse::<i64>()?;

        if range.iter().any(|x| id >= x[0] && id <= x[1]) {
            counter += 1;
        }
    }

    println!("Part 1: {}", counter);
    Ok(())
}

pub fn part_2() -> Result<()> {
    let spl = DATA.split("\r\n\r\n").collect::<Vec<_>>();
    let mut vec = vec![];

    for i in spl[0].lines() {
        vec.push(
            i.split('-')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );

        vec.sort_by_key(|x| x[0]);
        let mut ind = 0;

        while ind < vec.len() - 1 {
            if vec[ind][1] >= vec[ind + 1][0] && vec[ind][1] <= vec[ind + 1][1] {
                vec[ind][1] = vec[ind + 1][1];
                vec.remove(ind + 1);
            } else if vec[ind][1] >= vec[ind + 1][1] {
                vec.remove(ind + 1);
            } else {
                ind += 1;
            }
        }
    }

    let mut sum = 0;
    for i in vec {
        sum += i[1] - i[0] + 1;
    }

    println!("Part 2: {}", sum);
    Ok(())
}
