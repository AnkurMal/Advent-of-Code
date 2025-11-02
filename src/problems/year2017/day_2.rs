use anyhow::Result;

const DATA: &str = include_str!("../../data/year2017/day_2.txt");

pub fn part_1() -> Result<()> {
    let mut sum = 0;
    for data in DATA.lines() {
        let vec = data
            .split('\t')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        sum += vec.iter().max().unwrap() - vec.iter().min().unwrap();
    }

    println!("Part 1: {sum}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut sum = 0;
    for data in DATA.lines() {
        let mut vec = data
            .split('\t')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        vec.sort();
        for (ind, i) in vec.iter().enumerate() {
            for j in vec.iter().skip(ind + 1) {
                if j % i == 0 {
                    sum += j / i;
                }
            }
        }
    }

    println!("Part 2: {sum}");
    Ok(())
}
