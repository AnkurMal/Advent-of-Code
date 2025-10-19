const DATA: &str = include_str!("../../data/year2024/day_10.txt");

pub fn part_1() {
    let mut count = 0;
    let (grid, width, height) = process_data();
    let trail_heads = get_trail_heads(&grid);

    for i in trail_heads.iter() {
        let mut p_vec = vec![*i];

        while !p_vec.is_empty() {
            let len = p_vec.len();
            for i in 0..len {
                let (x, y) = p_vec[i];
                let score = grid[y][x];
                if score == 9 {
                    count += 1;
                } else {
                    if y > 0 && grid[y - 1][x] == score + 1 && !p_vec.contains(&(x, y - 1)) {
                        p_vec.push((x, y - 1));
                    }
                    if y < height - 1 && grid[y + 1][x] == score + 1 && !p_vec.contains(&(x, y + 1))
                    {
                        p_vec.push((x, y + 1));
                    }
                    if x > 0 && grid[y][x - 1] == score + 1 && !p_vec.contains(&(x - 1, y)) {
                        p_vec.push((x - 1, y));
                    }
                    if x < width - 1 && grid[y][x + 1] == score + 1 && !p_vec.contains(&(x + 1, y))
                    {
                        p_vec.push((x + 1, y));
                    }
                }
            }

            for i in 0..len {
                p_vec.remove(len - i - 1);
            }
        }
    }

    println!("Part 1: {count}")
}

pub fn part_2() {
    let mut count = 0;
    let (grid, width, height) = process_data();
    let trail_heads = get_trail_heads(&grid);

    for i in trail_heads.iter() {
        let mut p_vec = vec![*i];

        while !p_vec.is_empty() {
            let len = p_vec.len();
            for i in 0..len {
                let (x, y) = p_vec[i];
                let score = grid[y][x];
                if score == 9 {
                    count += 1;
                } else {
                    if y > 0 && grid[y - 1][x] == score + 1 {
                        p_vec.push((x, y - 1));
                    }
                    if y < height - 1 && grid[y + 1][x] == score + 1 {
                        p_vec.push((x, y + 1));
                    }
                    if x > 0 && grid[y][x - 1] == score + 1 {
                        p_vec.push((x - 1, y));
                    }
                    if x < width - 1 && grid[y][x + 1] == score + 1 {
                        p_vec.push((x + 1, y));
                    }
                }
            }

            for i in 0..len {
                p_vec.remove(len - i - 1);
            }
        }
    }

    println!("Part 2: {count}")
}

fn process_data() -> (Vec<Vec<u32>>, usize, usize) {
    let grid = DATA
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let width = grid[0].len();
    let height = grid.len();

    (grid, width, height)
}

fn get_trail_heads(grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut trail_heads = vec![];

    for (y, i) in grid.iter().enumerate() {
        for (x, j) in i.iter().enumerate() {
            if *j == 0 {
                trail_heads.push((x, y));
            }
        }
    }

    trail_heads
}
