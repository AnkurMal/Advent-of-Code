const DATA: &str = include_str!("../../data/year2024/day_6.txt");

pub fn part_1() {
    let (mut x, mut y, mut dx, mut dy) = (0, 0, 0, 0);
    let mut visited = vec![];
    let mut lines = DATA.lines().collect::<Vec<&str>>();

    for (i, &data) in lines.iter().enumerate() {
        for j in 0..data.len() {
            let ch = data.chars().nth(j).unwrap();

            if ch != '.' && ch != '#' {
                if ch == '^' {
                    dy = -1;
                }
                if ch == '>' {
                    dx = 1;
                }
                if ch == 'v' {
                    dy = 1;
                }
                if ch == '<' {
                    dx = -1;
                }

                x = j as i32;
                y = i as i32;
                visited.push((x, y));
            }
        }
    }

    loop {
        if (x > 0 && x < (lines[0].len() - 1) as i32) && (y > 0 && y < (lines.len() - 1) as i32) {
            let next = lines[(y + dy) as usize]
                .chars()
                .nth((x + dx) as usize)
                .unwrap();

            if next == '#' {
                if dy == -1 {
                    dy = 0;
                    dx = 1;
                } else if dy == 1 {
                    dy = 0;
                    dx = -1;
                } else if dx == -1 {
                    dx = 0;
                    dy = -1;
                } else if dx == 1 {
                    dx = 0;
                    dy = 1;
                }
            }
        }

        y += dy;
        x += dx;

        if (x < 0 || x >= lines[0].len() as i32) || (y < 0 || y >= lines.len() as i32) {
            break;
        }
        if !visited.contains(&(x, y)) {
            visited.push((x, y));
        }
    }

    println!("Part 1: {}", visited.len());
}

pub fn part_2() {
    let mut grid = DATA
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let row = grid.len();
    let col = grid[0].len();
    let mut count = 0;

    let (mut x, mut y, mut dx, mut dy) = (0i32, 0i32, 0i32, 0i32);
    let (mut x_copy, mut y_copy, mut dx_copy, mut dy_copy) = (0i32, 0i32, 0i32, 0i32);
    let (mut o_x, mut o_y) = (0, 0);

    'outer: for (i, vec) in grid.iter().enumerate() {
        for (j, char) in vec.iter().enumerate() {
            match char {
                '^' => dy = -1,
                '>' => dx = 1,
                'v' => dy = 1,
                '<' => dx = -1,
                _ => (),
            }

            if *char != '.' && *char != '#' && *char != '0' {
                x = j as i32;
                y = i as i32;
                (x_copy, y_copy, dx_copy, dy_copy) = (x, y, dx, dy);

                break 'outer;
            }
        }
    }

    let (mut prev_i, mut prev_j) = (0, 0);
    for i in 0..row {
        for j in 0..col {
            let mut loop_count = 0;
            (x, y, dx, dy) = (x_copy, y_copy, dx_copy, dy_copy);

            if grid[i][j] == '.' {
                grid[prev_i][prev_j] = '.';
                grid[i][j] = '0';
                (prev_i, prev_j) = (i, j);
            } else {
                continue;
            }

            loop {
                if ((x + dx) >= 0 && (x + dx) < col as i32)
                    && ((y + dy) >= 0 && (y + dy) < row as i32)
                {
                    let next = grid[(y + dy) as usize][(x + dx) as usize];

                    if next == '#' || next == '0' {
                        match (dx, dy) {
                            (0, -1) => {
                                dx = 1;
                                dy = 0;
                            }
                            (0, 1) => {
                                dx = -1;
                                dy = 0;
                            }
                            (-1, 0) => {
                                dx = 0;
                                dy = -1;
                            }
                            (1, 0) => {
                                dx = 0;
                                dy = 1;
                            }
                            _ => (),
                        }
                        continue;
                    }
                }

                y += dy;
                x += dx;

                loop_count += 1;
                if loop_count > 25000 {
                    count += 1;
                    break;
                }

                if (x < 0 || x >= col as i32) || (y < 0 || y >= row as i32) {
                    break;
                }
            }
        }
    }

    println!("Part 2: {count}");
}
