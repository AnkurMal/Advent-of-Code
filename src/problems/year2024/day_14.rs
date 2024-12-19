use regex::Regex;

const DATA: &str = include_str!("../../data/year2024/day_14.txt");
const COORDS: [i32; 2] = [101, 103];

pub fn part_1() {
    let mid = [(COORDS[0]-1)/2, (COORDS[1]-1)/2];
    let mut res = [0; 4];
    let re = Regex::new(r"p=(\d+,\d+) v=(-?\d+,-?\d+)").unwrap();

    for i in re.captures_iter(DATA) {
        let final_pos = i[1].split(',').zip(i[2].split(',')).zip(COORDS).map(|((pos, vel), coord)| {
            (pos.parse::<i32>().unwrap() + 100*vel.parse::<i32>().unwrap()).rem_euclid(coord)
        }).collect::<Vec<i32>>();

        if final_pos[0]<mid[0] && final_pos[1]<mid[1] {
            res[0] += 1;
        }
        else if final_pos[0]>mid[0] && final_pos[1]<mid[1] {
            res[1] += 1;
        }
        else if final_pos[0]<mid[0] && final_pos[1]>mid[1] {
            res[2] += 1;
        }
        else if final_pos[0]>mid[0] && final_pos[1]>mid[1] {
            res[3] += 1;
        }
    }

    println!("Part 1: {}", res.iter().product::<i32>());
}

pub fn part_2() {
    let mut count = 1;

    let re = Regex::new(r"p=(\d+,\d+) v=(-?\d+,-?\d+)").unwrap();
    let tree = Regex::new(r"1{10}").unwrap();

    let mut pos_vec = vec![];
    let mut vel_vec = vec![];
    
    for i in re.captures_iter(DATA) {
        let pos = i[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let vel = i[2].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        pos_vec.push(pos);
        vel_vec.push(vel);
    }

    'outer: loop {
        let mut st = [['.'; COORDS[0] as usize]; COORDS[1] as usize];
        for (pos, vec) in pos_vec.iter_mut().zip(vel_vec.iter()) {
            pos[0] = (pos[0]+vec[0]).rem_euclid(COORDS[0]);
            pos[1] = (pos[1]+vec[1]).rem_euclid(COORDS[1]);

            st[pos[1] as usize][pos[0] as usize] = '1';
        }

        for i in st {
            let ans = i.iter().collect::<String>();
            if tree.is_match(&ans) {
                break 'outer;
            }
        }

        count += 1;
    }
    
    println!("Part 2: {}", count);
}