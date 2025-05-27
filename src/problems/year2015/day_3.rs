use std::collections::HashSet;

const DATA: &str = include_str!("../../data/year2015/day_3.txt");

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Grid {
    x: i32,
    y: i32
}

pub fn part_1() {
    let mut grid = Grid {x: 0, y: 0};
    let mut set = HashSet::new();

    for i in DATA.chars() {
        if i=='>' {
            grid.x += 1;
        }
        else if i=='<' {
            grid.x -= 1;
        }
        else if i=='^' {
            grid.y -= 1;
        }
        else {
            grid.y += 1;
        } 

        set.insert(grid);
    }

    println!("Part 1: {}", set.len());
}

pub fn part_2() {
    let mut santa = Grid {x: 0, y: 0};
    let mut robot = Grid {x: 0, y: 0};
    let mut set = HashSet::new();

    for (i, ch) in DATA.chars().enumerate() {
        let grid = if i%2==0 {
            &mut santa
        }
        else {
            &mut robot
        };

        if ch=='>' {
            grid.x += 1;
        }
        else if ch=='<' {
            grid.x -= 1;
        }
        else if ch=='^' {
            grid.y -= 1;
        }
        else {
            grid.y += 1;
        } 

        set.insert(*grid);
    }

    println!("Part 2: {}", set.len());
}