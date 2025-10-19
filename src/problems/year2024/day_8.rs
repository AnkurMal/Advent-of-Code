const DATA: &str = include_str!("../../data/year2024/day_8.txt");

pub fn part_1() {
    let line = DATA.lines().collect::<Vec<&str>>();
    let line_len = line.len() as i32;
    let st_len = line[0].len() as i32;

    let mut coord = vec![];

    for (ind_i, &i) in line.iter().enumerate() {
        for (ind_ch_i, ch_i) in i.chars().enumerate() {
            if ch_i != '.' {
                for (ind_j, &j) in line.iter().enumerate().skip(ind_i + 1) {
                    for (ind_ch_j, ch_j) in j.chars().enumerate() {
                        if ch_i == ch_j {
                            let col_dist = (ind_j - ind_i) as i32;
                            let row_dist = ind_ch_i as i32 - ind_ch_j as i32;

                            let up_coord = [ind_ch_i as i32 + row_dist, ind_i as i32 - col_dist];
                            let down_coord = [ind_ch_j as i32 - row_dist, ind_j as i32 + col_dist];

                            if !coord.contains(&up_coord)
                                && up_coord[1] >= 0
                                && up_coord[0] < st_len
                                && up_coord[0] >= 0
                            {
                                coord.push(up_coord);
                            }
                            if !coord.contains(&down_coord)
                                && down_coord[1] < line_len
                                && down_coord[0] < st_len
                                && down_coord[0] >= 0
                            {
                                coord.push(down_coord);
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", coord.len())
}

pub fn part_2() {
    let line = DATA.lines().collect::<Vec<&str>>();
    let line_len = line.len() as i32;
    let st_len = line[0].len() as i32;

    let mut coord = vec![];

    for (ind_i, &i) in line.iter().enumerate() {
        for (ind_ch_i, ch_i) in i.chars().enumerate() {
            if ch_i != '.' {
                if !coord.contains(&[ind_ch_i as i32, ind_i as i32]) {
                    coord.push([ind_ch_i as i32, ind_i as i32]);
                }
                for (ind_j, &j) in line.iter().enumerate().skip(ind_i + 1) {
                    for (ind_ch_j, ch_j) in j.chars().enumerate() {
                        if ch_i == ch_j {
                            let col_dist = (ind_j - ind_i) as i32;
                            let row_dist = ind_ch_i as i32 - ind_ch_j as i32;

                            let mut up_coord = [ind_ch_i as i32, ind_i as i32];
                            let mut down_coord = [ind_ch_j as i32, ind_j as i32];

                            loop {
                                let (mut up_check, mut down_check) = (false, false);

                                up_coord = [up_coord[0] + row_dist, up_coord[1] - col_dist];
                                down_coord = [down_coord[0] - row_dist, down_coord[1] + col_dist];

                                if up_coord[1] >= 0 && up_coord[0] < st_len && up_coord[0] >= 0 {
                                    up_check = true;
                                    if !coord.contains(&up_coord) {
                                        coord.push(up_coord);
                                    }
                                }
                                if down_coord[1] < line_len
                                    && down_coord[0] < st_len
                                    && down_coord[0] >= 0
                                {
                                    down_check = true;
                                    if !coord.contains(&down_coord) {
                                        coord.push(down_coord);
                                    }
                                }
                                if !up_check && !down_check {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", coord.len())
}
