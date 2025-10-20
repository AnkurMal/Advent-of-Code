use anyhow::Result;

const DATA: &str = include_str!("../../data/year2021/day_4.txt");

#[derive(Debug, Clone, Copy)]
struct Board {
    num: i32,
    marked: bool,
}

impl Board {
    fn new(num: i32) -> Self {
        Self { num, marked: false }
    }
}

pub fn part_1() -> Result<()> {
    let vec = DATA.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut bingo = vec![];

    let num_vec = vec[0].split(',').map(|x| x.parse::<i32>().unwrap());
    for items in vec.iter().skip(1) {
        let mut bin = vec![];

        for i in items.lines() {
            let int = i
                .split(' ')
                .filter_map(|x| {
                    if let Ok(num) = x.trim().parse() {
                        Some(Board::new(num))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Board>>();

            bin.push(int);
        }

        bingo.push(bin);
    }

    let mut last = 0;
    'outer: for i in num_vec {
        for bin in bingo.iter_mut() {
            for col in bin.iter_mut() {
                for board in col.iter_mut() {
                    if i == board.num {
                        board.marked = true;
                        last = i;
                    }
                }
            }
        }

        for bin in &bingo {
            if (has_complete_row(bin) || has_complete_column(bin)) {
                let mut unm_sum = 0;
                for fir in bin {
                    for item in fir {
                        if (!item.marked) {
                            unm_sum += item.num;
                        }
                    }
                }

                println!("Part 1: {}", unm_sum * last);
                break 'outer;
            }
        }
    }

    Ok(())
}

pub fn part_2() -> Result<()> {
    let vec = DATA.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut bingo = vec![];

    let num_vec = vec[0].split(',').map(|x| x.parse::<i32>().unwrap());
    for items in vec.iter().skip(1) {
        let mut bin = vec![];

        for i in items.lines() {
            let int = i
                .split(' ')
                .filter_map(|x| {
                    if let Ok(num) = x.trim().parse() {
                        Some(Board::new(num))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Board>>();

            bin.push(int);
        }

        bingo.push(bin);
    }

    let mut last = 0;
    'outer: for i in num_vec {
        for bin in bingo.iter_mut() {
            for col in bin.iter_mut() {
                for board in col.iter_mut() {
                    if i == board.num {
                        board.marked = true;
                        last = i;
                    }
                }
            }
        }

        if bingo.len() > 1 {
            bingo.retain(|x| !has_complete_row(x) && !has_complete_column(x));
        } else if (has_complete_row(&bingo[0]) || has_complete_column(&bingo[0])) {
            let mut unm_sum = 0;
            for fir in &bingo[0] {
                for item in fir {
                    if (!item.marked) {
                        unm_sum += item.num;
                    }
                }
            }

            println!("Part 2: {}", unm_sum * last);
            break 'outer;
        }
    }

    Ok(())
}

fn has_complete_row(board: &Vec<Vec<Board>>) -> bool {
    for row in board {
        if row.iter().all(|x| x.marked) {
            return true;
        }
    }
    false
}

fn has_complete_column(board: &Vec<Vec<Board>>) -> bool {
    for i in 0..5 {
        let mut mat = true;
        for bin in board {
            if (!bin[i].marked) {
                mat = false;
                break;
            }
        }

        if mat {
            return true;
        }
    }

    false
}
