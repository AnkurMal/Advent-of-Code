const DATA: &str = include_str!("../../data/year2024/day_15.txt");

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize
}

pub fn part_1() {
    let mut sum = 0;
    let lines = DATA.lines().collect::<Vec<&str>>();
    let (maze, steps) = lines.
    split_at(lines.iter().position(|&x| x.is_empty())
    .unwrap());

    let mut maze = process_data(maze);
    let mut coord = robot_coord(&maze);

    for i in steps.iter().flat_map(|x| x.chars()) {
        match i {
            '<' => robot_move_part_1(&mut maze, &mut coord, -1, 0),
            '>' => robot_move_part_1(&mut maze, &mut coord, 1, 0),
            '^' => robot_move_part_1(&mut maze, &mut coord, 0, -1),
            'v' => robot_move_part_1(&mut maze, &mut coord, 0, 1),
            _   => ()
        }
    }

    for (i, row) in maze.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char=='O' {
                sum += 100*i + j;
            }
        }
    }

    println!("Part 1: {sum}");
}

fn robot_move_part_1(maze: &mut [Vec<char>], coord: &mut Coord, dx: i32, dy: i32) {
    let mut crd = *coord;
    crd.x = (crd.x as i32 + dx) as usize;
    crd.y = (crd.y as i32 + dy) as usize;

    if maze[crd.y][crd.x] == '.' {
        maze[crd.y][crd.x] = '@';
        maze[coord.y][coord.x] = '.';
        *coord = crd;
    }
    else {
        while maze[crd.y][crd.x] != '.' && maze[crd.y][crd.x] != '#' {
            crd.x = (crd.x as i32 + dx) as usize;
            crd.y = (crd.y as i32 + dy) as usize;
        }
        if maze[crd.y][crd.x] == '.' {
            maze[coord.y][coord.x] = '.';
            coord.x = (coord.x as i32 + dx) as usize;
            coord.y = (coord.y as i32 + dy) as usize;

            maze[coord.y][coord.x] = '@';
            maze[crd.y][crd.x] = 'O';
        }
    }
}


fn robot_coord(maze: &[Vec<char>]) -> Coord {
    let mut coord = Coord {x: 0, y: 0};
    'outer: for (i, row) in maze.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            if *char=='@' {
                coord = Coord {x: j, y: i};
                break 'outer;
            }
        }
    }

    coord
}

fn process_data(maze: &[&str]) -> Vec<Vec<char>> {
    let grid = maze
    .iter()
    .map(|&x| x.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    grid
}