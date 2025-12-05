use anyhow::Result;

const DATA: &str = include_str!("../../data/year2025/day_4.txt");

pub fn part_1() -> Result<()> {
    let lines = DATA.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();
    let mut total = 0;
    let row = lines[0].len();
    let col = lines.len();

    for i in 0..col {
        for j in 0..row {
            let mut counter = 0;

            if lines[i][j] == b'@' {
                if i > 0 && j > 0 && lines[i - 1][j - 1] == b'@' {
                    counter += 1;
                }
                if i > 0 && lines[i - 1][j] == b'@' {
                    counter += 1;
                }
                if i > 0 && j < row - 1 && lines[i - 1][j + 1] == b'@' {
                    counter += 1;
                }
                if j > 0 && lines[i][j - 1] == b'@' {
                    counter += 1;
                }
                if j < row - 1 && lines[i][j + 1] == b'@' {
                    counter += 1;
                }
                if i < col - 1 && j > 0 && lines[i + 1][j - 1] == b'@' {
                    counter += 1;
                }
                if i < col - 1 && lines[i + 1][j] == b'@' {
                    counter += 1;
                }
                if i < col - 1 && j < row - 1 && lines[i + 1][j + 1] == b'@' {
                    counter += 1;
                }

                if counter < 4 {
                    total += 1;
                }
            }
        }
    }

    println!("Part 1: {total}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut lines = DATA
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let row = lines[0].len();
    let col = lines.len();
    let mut sum = 0;

    loop {
        let mut total = 0;

        for i in 0..col {
            for j in 0..row {
                let mut counter = 0;

                if lines[i][j] == '@' {
                    if i > 0 && j > 0 && lines[i - 1][j - 1] == '@' {
                        counter += 1;
                    }
                    if i > 0 && lines[i - 1][j] == '@' {
                        counter += 1;
                    }
                    if i > 0 && j < row - 1 && lines[i - 1][j + 1] == '@' {
                        counter += 1;
                    }
                    if j > 0 && lines[i][j - 1] == '@' {
                        counter += 1;
                    }
                    if j < row - 1 && lines[i][j + 1] == '@' {
                        counter += 1;
                    }
                    if i < col - 1 && j > 0 && lines[i + 1][j - 1] == '@' {
                        counter += 1;
                    }
                    if i < col - 1 && lines[i + 1][j] == '@' {
                        counter += 1;
                    }
                    if i < col - 1 && j < row - 1 && lines[i + 1][j + 1] == '@' {
                        counter += 1;
                    }

                    if counter < 4 {
                        lines[i][j] = '.';
                        total += 1;
                    }
                }
            }
        }

        if total == 0 {
            break;
        }
        sum += total;
    }

    println!("Part 2: {sum}");
    Ok(())
}
