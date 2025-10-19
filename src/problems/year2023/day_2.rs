const DATA: &str = include_str!("../../data/year2023/day_2.txt");

#[derive(Debug)]
struct Cube {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cube {
    fn new(red: u32, green: u32, blue: u32) -> Cube {
        Cube { red, green, blue }
    }

    fn reset(&mut self) {
        self.red = 0;
        self.green = 0;
        self.blue = 0;
    }

    fn product(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn part_1() {
    let mut id_vec = Vec::new();
    let mut count = 0;
    let max = Cube::new(12, 13, 14);
    let lines = DATA.lines();

    for i in lines {
        let mut smaller = true;
        let mut cube = Cube::new(0, 0, 0);
        let str = i.split(": ").last().unwrap();
        let token: Vec<&str> = str.split("; ").collect();
        count += 1;

        for j in token.iter() {
            let split: Vec<&str> = j.split(" ").collect();

            for i in (1..split.len()).step_by(2) {
                let item = split.get(i).unwrap();
                if item.contains("red") {
                    cube.red = split.get(i - 1).unwrap().parse().unwrap();
                } else if item.contains("green") {
                    cube.green = split.get(i - 1).unwrap().parse().unwrap();
                }
                if item.contains("blue") {
                    cube.blue = split.get(i - 1).unwrap().parse().unwrap();
                }
            }

            if cube.red > max.red || cube.green > max.green || cube.blue > max.blue {
                smaller = false;
                continue;
            }
            cube.reset();
        }
        if smaller {
            id_vec.push(count);
        }
    }
    let sum: i32 = id_vec.iter().sum();
    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let mut sum = 0;
    let lines = DATA.lines();

    for i in lines {
        let mut cube = Cube::new(0, 0, 0);
        let str = i.split(": ").last().unwrap();
        let token: Vec<&str> = str.split("; ").collect();

        for j in token.iter() {
            let split: Vec<&str> = j.split(" ").collect();

            for i in (1..split.len()).step_by(2) {
                let item = split.get(i).unwrap();
                if item.contains("red") {
                    let red: u32 = split.get(i - 1).unwrap().parse().unwrap();
                    if red > cube.red {
                        cube.red = red;
                    }
                } else if item.contains("green") {
                    let green: u32 = split.get(i - 1).unwrap().parse().unwrap();
                    if green > cube.green {
                        cube.green = green;
                    }
                }
                if item.contains("blue") {
                    let blue: u32 = split.get(i - 1).unwrap().parse().unwrap();
                    if blue > cube.blue {
                        cube.blue = blue;
                    }
                }
            }
        }
        sum += cube.product();
        cube.reset();
    }
    println!("Part 2: {}", sum);
}
