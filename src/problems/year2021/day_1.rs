use anyhow::Result;

const DATA: &str = include_str!("../../data/year2021/day_1.txt");

pub fn part_1() -> Result<()> {
    let data = String::new();
    let mut vec = vec![];
    let mut counter = 0;

    for i in DATA.lines() {
        vec.push(i.trim().parse::<i32>()?);
    }

    for i in 0..vec.len() {
        if (i > 0 && vec[i] > vec[i - 1]) {
            counter += 1;
        }
    }

    println!("Part 1: {}", counter);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let data = String::new();
    let mut vec = vec![];
    let mut sum = vec![];
    let mut counter = 0;

    for i in DATA.lines() {
        vec.push(i.trim().parse::<i32>()?);
    }

    for i in vec.windows(3) {
        sum.push(i.iter().sum::<i32>());
    }

    for i in 0..sum.len() {
        if (i > 0 && sum[i] > sum[i - 1]) {
            counter += 1;
        }
    }

    println!("Part 2: {}", counter);

    Ok(())
}
