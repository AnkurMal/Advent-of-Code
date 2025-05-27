const DATA: &str = include_str!("../../data/year2015/day_2.txt");

pub fn part_1() {
    let mut sum = 0;

    for line in DATA.lines() {
        let spl = line.split('x').collect::<Vec<&str>>();

        let l = spl[0].parse::<i32>().unwrap();
        let w = spl[1].parse::<i32>().unwrap();
        let h = spl[2].parse::<i32>().unwrap();

        sum += 2*l*w + 2*w*h + 2*h*l;
        sum += [l*w, w*h, h*l].iter().min().unwrap();
    }

    println!("Part 1: {sum}");
}

pub fn part_2() {
    let mut sum = 0;

    for line in DATA.lines() {
        let spl = line.split('x').collect::<Vec<&str>>();

        let l = spl[0].parse::<i32>().unwrap();
        let w = spl[1].parse::<i32>().unwrap();
        let h = spl[2].parse::<i32>().unwrap();

        sum += l*w*h + [2*(l+w), 2*(w+h), 2*(h+l)].iter().min().unwrap();
    }

    println!("Part 2: {sum}");
}