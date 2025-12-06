use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_6.txt");

pub fn part_1() -> Result<()> {
    let len = DATA.lines().count();
    let mut total = 0;

    let vec = DATA
        .lines()
        .take(len - 1)
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ops = DATA.lines().last().unwrap().split_whitespace();

    for (i, op) in ops.into_iter().enumerate() {
        if op == "+" {
            let mut sum = 0;
            for v in &vec {
                sum += v[i];
            }

            total += sum;
        } else if op == "*" {
            let mut product = 1;
            for v in &vec {
                product *= v[i];
            }

            total += product;
        }
    }

    println!("Part 1: {}", total);
    Ok(())
}

pub fn part_2() -> Result<()> {
    let len = DATA.lines().count();
    let lines = DATA
        .lines()
        .take(len - 1)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total = 0;
    let ops = DATA.lines().last().unwrap().as_bytes();

    let mut sym = ' ';
    let mut tot = 0;

    for i in 0..lines[0].len() {
        if ops[i] != b' ' {
            total += tot;
        }

        if ops[i] == b'+' {
            tot = 0;
            sym = '+';
        } else if ops[i] == b'*' {
            tot = 1;
            sym = '*';
        }

        let mut num = String::new();
        for j in &lines {
            num.push(j[i]);
        }

        if let Ok(num) = num.trim().parse::<i64>() {
            if sym == '+' {
                tot += num;
            } else {
                tot *= num;
            }
        }
    }

    println!("Part 2: {}", total + tot);
    Ok(())
}
