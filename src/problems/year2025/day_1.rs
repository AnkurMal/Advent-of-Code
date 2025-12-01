use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_1.txt");

pub fn part_1() -> Result<()> {
    let mut point = 50;
    let mut counter = 0;

    for line in DATA.lines() {
        if &line[..1] == "L" {
            point = (point - line[1..].parse::<i32>()?).rem_euclid(100);
        } else {
            point = (point + line[1..].parse::<i32>()?) % 100;
        }

        if point == 0 {
            counter += 1;
        }
    }

    println!("Part 1: {counter}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut point = 50;
    let mut counter = 0;

    for line in DATA.lines() {
        let num = line[1..].parse::<i32>()?;
        let inc: i32 = if &line[..1] == "L" { -1 } else { 1 };

        for _ in 0..num {
            point = (point + inc).rem_euclid(100);
            if point == 0 {
                counter += 1;
            }
        }
    }

    println!("Part 2: {counter}");
    Ok(())
}
