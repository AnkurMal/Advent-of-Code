use anyhow::Result;

const DATA: &str = include_str!("../../data/year2021/day_2.txt");

pub fn part_1() -> Result<()> {
    let (mut hor, mut dep) = (0, 0);

    for i in DATA.lines() {
        let vec = i.split(' ').collect::<Vec<&str>>();
        let num = vec[1].trim().parse::<i32>()?;

        match vec[0] {
            "forward" => hor += num,
            "down" => dep += num,
            _ => dep -= num,
        }
    }

    println!("Part 1: {}", hor * dep);

    Ok(())
}

pub fn part_2() -> Result<()> {
    let (mut hor, mut dep, mut aim) = (0, 0, 0);

    for i in DATA.lines() {
        let vec = i.split(' ').collect::<Vec<&str>>();
        let num = vec[1].trim().parse::<i32>()?;

        match vec[0] {
            "forward" => {
                hor += num;
                dep += aim * num;
            }
            "down" => aim += num,
            _ => aim -= num,
        }
    }

    println!("Part 1: {}", hor * dep);

    Ok(())
}
