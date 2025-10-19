use std::collections::HashMap;

const DATA: &str = include_str!("../../data/year2024/day_11.txt");

pub fn part_1() {
    let mut data = process_data();
    let mut map = HashMap::new();
    let mut sum = 0;

    for &i in data.iter() {
        sum += parse(i, 25, &mut map);
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let mut data = process_data();
    let mut map = HashMap::new();
    let mut sum = 0;

    for &i in data.iter() {
        sum += parse(i, 75, &mut map);
    }

    println!("Part 2: {}", sum);
}

fn parse(data: u64, level: u8, map: &mut HashMap<(u64, u8), usize>) -> usize {
    if level == 0 {
        1
    } else if let Some(&cached_count) = map.get(&(data, level)) {
        cached_count
    } else {
        let mut count = 0;
        if data == 0 {
            count += parse(1, level - 1, map);
        } else {
            let digits = count_digits(data);
            if digits.is_multiple_of(2) {
                let left = data / 10u64.pow(digits / 2);
                let right = data % 10u64.pow(digits / 2);

                count += parse(left, level - 1, map);
                count += parse(right, level - 1, map);
            } else {
                count += parse(data * 2024, level - 1, map);
            }
        }

        map.insert((data, level), count);
        count
    }
}

fn count_digits(digit: u64) -> u32 {
    let mut sum = 0;
    let mut dig = digit;
    while dig > 0 {
        dig /= 10;
        sum += 1;
    }
    sum
}

fn process_data() -> Vec<u64> {
    DATA.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
