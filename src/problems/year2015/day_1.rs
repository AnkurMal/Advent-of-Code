const DATA: &str = include_str!("../../data/year2015/day_1.txt");

pub fn part_1() {
    let mut sum = 0;

    for i in DATA.chars() {
        if (i=='(') {
            sum += 1;
        }
        else {
            sum -= 1;
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let mut sum = 0;

    for (i, val) in DATA.chars().enumerate() {
        if (val=='(') {
            sum += 1;
        }
        else {
            sum -= 1;
        }

        if (sum==-1) {
            println!("Part 2: {}", i+1);
            return;
        }
    }
}