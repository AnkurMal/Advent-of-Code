use regex::Regex;

const DATA: &str = include_str!("../../data/year2024/day_13.txt");

pub fn part_1() {
    let re =
        Regex::new(r".+X\+(\d+), Y\+(\d+).*\n.+X\+(\d+), Y\+(\d+).*\n.+X=(\d+), Y=(\d+)").unwrap();
    let mut sum = 0;

    for i in re.captures_iter(DATA) {
        let eq = [&i[1], &i[2], &i[3], &i[4], &i[5], &i[6]];
        let eq = eq.map(|x| x.parse::<i64>().unwrap());

        let dividend_y = eq[0] * eq[5] - eq[1] * eq[4];
        let divisor_y = eq[0] * eq[3] - eq[1] * eq[2];

        let dividend_x = eq[2] * eq[5] - eq[3] * eq[4];
        let divisor_x = -divisor_y;

        if dividend_y.rem_euclid(divisor_y) != 0 || dividend_x.rem_euclid(divisor_x) != 0 {
            continue;
        }

        let y = dividend_y / divisor_y;
        let x = dividend_x / divisor_x;
        sum += x * 3 + y;
    }

    println!("Part 1: {sum}")
}

pub fn part_2() {
    let re =
        Regex::new(r".+X\+(\d+), Y\+(\d+).*\n.+X\+(\d+), Y\+(\d+).*\n.+X=(\d+), Y=(\d+)").unwrap();
    let mut sum = 0;

    for i in re.captures_iter(DATA) {
        let eq = [&i[1], &i[2], &i[3], &i[4], &i[5], &i[6]];
        let mut eq = eq.map(|x| x.parse::<i64>().unwrap());
        eq[4] += 10000000000000;
        eq[5] += 10000000000000;

        let dividend_y = eq[0] * eq[5] - eq[1] * eq[4];
        let divisor_y = eq[0] * eq[3] - eq[1] * eq[2];

        let dividend_x = eq[2] * eq[5] - eq[3] * eq[4];
        let divisor_x = -divisor_y;

        if dividend_y.rem_euclid(divisor_y) != 0 || dividend_x.rem_euclid(divisor_x) != 0 {
            continue;
        }

        let y = dividend_y / divisor_y;
        let x = dividend_x / divisor_x;
        sum += x * 3 + y;
    }

    println!("Part 2: {sum}")
}
