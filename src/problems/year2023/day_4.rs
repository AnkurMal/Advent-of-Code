const DATA: &str = include_str!("../../data/year2023/day_4.txt");

pub fn part_1() {
    let lines: Vec<&str> = DATA.lines().collect();
    let mut sum =  0;
    
    for line in lines {
        let vec: Vec<&str> = line.split(" | ").collect();
        let mut points = 0;

        let lvec: Vec<&str> = vec[0].split(": ").collect();
        let lvec: Vec<&str> = lvec[1].split(' ').filter(|x| !x.is_empty()).collect();
        let rvec: Vec<&str> = vec[1].split(' ').filter(|x| !x.is_empty()).collect();
        
        for lnum in lvec.iter() {
            if rvec.contains(lnum) {
                match points {
                    0 => {points = 1},
                    _ => {points *= 2}
                }
            }
        }
        sum += points;
    }
    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let lines: Vec<&str> = DATA.lines().collect();
    
    let mut nvec: Vec<usize> = vec![1; lines.len()];
    
    for (i, line) in lines.iter().enumerate() {
        let vec: Vec<&str> = line.split(" | ").collect();
        let mut points = 0;

        let lvec: Vec<&str> = vec[0].split(": ").collect();
        let lvec: Vec<&str> = lvec[1].split(' ').filter(|x| !x.is_empty()).collect();
        let rvec: Vec<&str> = vec[1].split(' ').filter(|x| !x.is_empty()).collect();
        
        for lnum in lvec.iter() {
            if rvec.contains(lnum) {
                points += 1;
            }
        }

        for _ in 0..nvec[i] {
            for item in nvec.iter_mut().take(i + points + 1).skip(i+1) {
                *item += 1;
            }
        }
    }
    let sum: usize = nvec.iter().sum();
    println!("Part 2: {}", sum);
}