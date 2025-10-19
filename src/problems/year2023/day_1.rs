const DATA: &str = include_str!("../../data/year2023/day_1.txt");

pub fn part_1() {
    let file = DATA.lines();
    let mut vec: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for i in file {
        for j in i.chars() {
            if j.is_numeric() {
                vec.push(j.to_digit(10).unwrap());
            }
        }
        let first = vec.first().unwrap();
        let last = vec.last().unwrap();
        sum += first * 10 + last;
        vec.clear();
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let file = DATA.lines();
    let mut sum: i32 = 0;

    let list = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];

    for i in file {
        let mut first = i.len();
        let mut fnum = "";

        let mut last = 0;
        let mut lnum = "";

        for j in list {
            let fi = i.find(j);
            if let Some(index) = fi
                && index <= first
            {
                first = index;
                fnum = j;
            }

            let li = i.rfind(j);
            if let Some(index) = li
                && index >= last
            {
                last = index;
                lnum = j;
            }
        }

        sum += get_num(fnum) * 10 + get_num(lnum);
    }
    println!("Part 2: {}", sum);
}

fn get_num(num_str: &str) -> i32 {
    match num_str {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
