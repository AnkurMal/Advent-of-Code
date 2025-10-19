const DATA: &str = include_str!("../../data/year2024/day_9.txt");

pub fn part_1() {
    let mut sum = 0;
    let (mut block, mut id) = parse_block();

    let mut len = block.len() - 1;
    'outer: for ind in 0..block.len() {
        if block[ind] == "." {
            while block[len] == "." {
                len -= 1;
                if len <= ind {
                    break 'outer;
                }
            }
            block.swap(ind, len);
        }
    }

    for (ind, st) in block.iter().enumerate() {
        if st == "." {
            break;
        }
        sum += st.parse::<usize>().unwrap() * ind;
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let mut sum: u64 = 0;
    let (mut id, mut space) = (vec![], vec![]);

    for (ind, ch) in DATA.chars().enumerate() {
        match ind % 2 == 0 {
            true => id.push(ch.to_digit(10).unwrap()),
            false => space.push(ch.to_digit(10).unwrap()),
        }
    }

    let mut id_pos: Vec<u32> = Vec::with_capacity(id.len());
    let mut space_pos: Vec<u32> = Vec::with_capacity(id.len());

    space_pos.push(id[0]);
    id_pos.push(0);
    for (ind, (i, j)) in id.iter().skip(1).zip(space.iter()).enumerate() {
        space_pos.push(space_pos[ind] + i + j);
        id_pos.push(space_pos[ind + 1] - id[ind + 1]);
    }

    'outer: for (ind, i) in id.iter().enumerate().skip(1).rev() {
        for (ind_j, j) in space.iter_mut().take(ind).enumerate() {
            if *j >= *i {
                *j -= i;
                for k in (0..*i) {
                    sum += (space_pos[ind_j] + k) as u64 * ind as u64;
                }
                space_pos[ind_j] += *i;
                continue 'outer;
            }
        }
        for k in (0..*i) {
            sum += (id_pos[ind] + k) as u64 * ind as u64;
        }
    }

    println!("Part 2: {}", sum);
}

fn parse_block() -> (Vec<String>, i32) {
    let mut block = vec![];
    let mut id = 0;

    for (ind, ch) in DATA.chars().enumerate() {
        if ind % 2 == 0 {
            let fr = format!("{}", id);
            (0..ch.to_digit(10).unwrap()).for_each(|_| block.push(fr.clone()));
            id += 1;
        } else {
            (0..ch.to_digit(10).unwrap()).for_each(|_| block.push(".".to_owned()));
        }
    }

    (block, id)
}
