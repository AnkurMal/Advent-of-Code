const DATA: &str = include_str!("../../data/year2024/day_7.txt");

pub fn part_1() {
    let mut ans = 0;
    let mut carry;

    for i in DATA.lines() {
        let num = parse_data(i);

        let sum = num.iter().skip(1).sum::<i64>();
        if sum == num[0] {
            ans += sum;
            continue;
        }

        let mut ops_len = num.len() - 2;
        let mut ops = vec!['*'; ops_len];
        let comp = vec!['+'; ops_len];

        while ops != comp {
            let mut res = num[1];
            for (i, ch) in num.iter().take(num.len()).skip(2).zip(ops.iter()) {
                match ch {
                    '+' => res += i,
                    '*' => res *= i,
                    _ => (),
                }
            }
            if res == num[0] {
                ans += res;
                break;
            }

            for i in ops.iter_mut() {
                carry = if *i == '+' { 1 } else { 0 };
                *i = if *i == '+' { '*' } else { '+' };
                if carry == 0 {
                    break;
                }
            }
        }
    }

    println!("Part 1: {ans}");
}

pub fn part_2() {
    let mut ans = 0;
    let mut carry;

    for i in DATA.lines() {
        let num = parse_data(i);

        let total = num
            .iter()
            .skip(1)
            .map(|x| x.to_string())
            .collect::<String>();
        let total = total.parse::<i64>().unwrap();
        if total == num[0] {
            ans += total;
            continue;
        }

        let mut ops_len = num.len() - 2;
        let mut ops = vec!["*"; ops_len];
        let comp = vec!["||"; ops_len];

        while ops != comp {
            let mut res = num[1];
            for (i, ch) in num.iter().take(num.len()).skip(2).zip(ops.iter()) {
                match *ch {
                    "+" => res += i,
                    "*" => res *= i,
                    _ => {
                        let st = format!("{}{}", res, i);
                        res = st.parse().unwrap();
                    }
                }
            }
            if res == num[0] {
                ans += res;
                break;
            }

            for i in ops.iter_mut() {
                carry = if *i == "||" { 1 } else { 0 };
                *i = if *i == "+" {
                    "||"
                } else if *i == "*" {
                    "+"
                } else {
                    "*"
                };
                if carry == 0 {
                    break;
                }
            }
        }
    }

    println!("Part 2: {ans}");
}

fn parse_data(data: &str) -> Vec<i64> {
    data.split_whitespace()
        .enumerate()
        .map(|(x, y)| {
            if x == 0 {
                let y = y.chars().take(y.len() - 1).collect::<String>();
                return y.parse::<i64>().unwrap();
            }
            y.parse::<i64>().unwrap()
        })
        .collect()
}
