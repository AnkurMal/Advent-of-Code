const DATA: &str = include_str!("../../data/year2023/day_6.txt");

pub fn part_1() {
    let lines: Vec<&str> = DATA.lines().collect();
    let mut err = 1;

    let time: Vec<i32> = lines[0]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let dist: Vec<i32> = lines[1]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    for i in 0..time.len() {
        for j in 1..time[i] {
            if (j * (time[i] - j)) > dist[i] {
                err *= time[i] - 2 * j + 1;
                break;
            }
        }
    }

    println!("Part 1: {}", err);
}

pub fn part_2() {
    let lines: Vec<&str> = DATA.lines().collect();

    let time: i64 = lines[0]
        .split(' ')
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let dist: i64 = lines[1]
        .split(' ')
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    for i in 1..time {
        if (i * (time - i)) > dist {
            println!("Part 2: {}", time - 2 * i + 1);
            break;
        }
    }
}
