const DATA: &str = include_str!("../../data/year2024/day_2.txt");

pub fn part_1() {
    let mut safe = 0;

    for i in DATA.lines() {
        let mut pos = true;
        let mut ok = true;
        let num = i.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for j in 0..num.len()-1 {
            let diff = num[j]-num[j+1];

            if j==0 && diff<0 {
                pos = false;
            }
            if (diff<0 && pos) || (diff>0 && !pos) || !(1..=3).contains(&diff.abs()) {
                ok = false;
                break;
            }
        }
        if ok {
            safe += 1;
        }
    }
    println!("Part 1: {safe}");
}

pub fn part_2() {
    let mut safe = 0;
    let mut num = vec![];
    let mut lines = DATA.lines();
    let mut line = lines.next();

    let mut ind = 0;
    let mut rem = None;

    while let Some(data) = line  {
        let mut pos = true;
        let mut ok = true;
        if rem.is_none() {
            num = data.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        }

        for j in 0..num.len()-1 {
            let diff = num[j]-num[j+1];

            if j==0 && diff<0 {
                pos = false;
            }
            if (diff<0 && pos) || (diff>0 && !pos) || !(1..=3).contains(&diff.abs()) {
                ok = false;
                break;
            }
        }
        if ok {
            safe += 1;
        }
        else if ind<=num.len(){
            if let Some(rem_str) = rem {
                num.insert(ind, rem_str);
                ind += 1;
            }
            rem = Some(num.remove(ind));
        }

        if ok || ind==num.len()+1 {
            line = lines.next();
            ind = 0;
            rem = None;
        }

        if ind==num.len() {
            ind += 1;
        }
    }
    
    println!("Part 2: {safe}");
}