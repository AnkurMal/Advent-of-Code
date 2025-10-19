use regex::Regex;

const DATA: &str = include_str!("../../data/year2024/day_3.txt");

pub fn part_1() {
    let mut sum = 0;
    let capt = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for pat in capt.captures_iter(DATA) {
        let num1 = pat[1].parse::<i32>().unwrap();
        let num2 = pat[2].parse::<i32>().unwrap();

        sum += num1 * num2;
    }

    println!("Part 1: {sum}")
}

pub fn part_2() {
    let mut sum = 0;
    let do_pattern = Regex::new(r"(?s)do\(\).*?don\'t\(\)").unwrap();
    let capt = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut data = format!("do(){DATA}don't()");

    for i in do_pattern.find_iter(data.as_str()).map(|x| x.as_str()) {
        for pat in capt.captures_iter(i) {
            let num1 = pat[1].parse::<i32>().unwrap();
            let num2 = pat[2].parse::<i32>().unwrap();

            sum += num1 * num2;
        }
    }

    println!("Part 2: {sum}")
}
