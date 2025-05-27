pub fn part_1() {
    let mut num = 1;

    loop {
        let key = format!("yzbqklnj{num}");
        let hash = format!("{:x}", md5::compute(key));

        if hash.starts_with("00000") {
            println!("Part 1: {}", num);
            break;
        }
        num += 1;
    }
}

pub fn part_2() {
    let mut num = 1;

    loop {
        let key = format!("yzbqklnj{num}");
        let hash = format!("{:x}", md5::compute(key));

        if hash.starts_with("000000") {
            println!("Part 2: {}", num);
            break;
        }
        num += 1;
    }
}