use anyhow::Result;

const DATA: &str = include_str!("../../data/year2017/day_1.txt");

pub fn part_1() -> Result<()> {
    let data = DATA.as_bytes();
    let mut sum: i32 = 0;

    for i in 0..data.len() {
        if data[i] == data[(i + 1) % data.len()] {
            sum += (data[i] - b'0') as i32;
        }
    }

    println!("Part 1: {sum}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let data = DATA.as_bytes();
    let mut sum: i32 = 0;

    for i in 0..data.len() {
        if data[i] == data[(i + data.len() / 2) % data.len()] {
            sum += (data[i] - b'0') as i32;
        }
    }

    println!("Part 2: {sum}");
    Ok(())
}
