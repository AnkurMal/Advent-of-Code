use regex::Regex;

const DATA: &str = include_str!("../../data/year2024/day_4.txt");

pub fn part_1() {
    let mut count = 0;
    let lines = DATA.lines().collect::<Vec<&str>>();

    let line_len = lines.len();
    let st_len = lines[0].len();

    let re_xmas: Regex = Regex::new("XMAS").unwrap();
    let re_samx: Regex = Regex::new("SAMX").unwrap();

    for i in 0..st_len {
        let mut st1 = String::new();
        let mut st2 = String::new();
        let mut st3 = String::new();

        for (ind, j) in lines.iter().enumerate() {
            st1.push(j.chars().nth(i).unwrap());

            if i <= st_len - 4 && ind < line_len - i {
                st2.push(j.chars().nth(i + ind).unwrap());
            }
            if i >= 3 && ind <= i {
                st3.push(j.chars().nth(i - ind).unwrap());
            }
        }

        count += re_xmas.find_iter(&st1).count() + re_samx.find_iter(&st1).count();
        count += re_xmas.find_iter(&st2).count() + re_samx.find_iter(&st2).count();
        count += re_xmas.find_iter(&st3).count() + re_samx.find_iter(&st3).count();
    }

    for i in 1..line_len {
        let mut st = String::new();
        let mut st1 = String::new();

        for j in 0..st_len {
            if i <= line_len - 4 && i < line_len - j {
                st.push(lines[i + j].chars().nth(j).unwrap());
                st1.push(lines[i + j].chars().nth(st_len - j - 1).unwrap());
            }
        }

        count += re_xmas.find_iter(&st).count() + re_samx.find_iter(&st).count();
        count += re_xmas.find_iter(&st1).count() + re_samx.find_iter(&st1).count();
    }

    for i in lines.iter() {
        count += re_xmas.find_iter(i).count() + re_samx.find_iter(i).count();
    }

    println!("Part 1: {count}");
}

pub fn part_2() {
    let mut count = 0;
    let lines = DATA.lines().collect::<Vec<&str>>();

    for i in 1..lines.len() - 1 {
        for j in 1..lines[0].len() - 1 {
            if lines[i].chars().nth(j).unwrap() == 'A' {
                let char_bb = lines[i - 1].chars().nth(j - 1).unwrap();
                let char_ba = lines[i - 1].chars().nth(j + 1).unwrap();

                let char_ab = lines[i + 1].chars().nth(j - 1).unwrap();
                let char_aa = lines[i + 1].chars().nth(j + 1).unwrap();

                if (char_bb == 'M' && char_ba == 'S' && char_ab == 'M' && char_aa == 'S')
                    || (char_bb == 'M' && char_ba == 'M' && char_ab == 'S' && char_aa == 'S')
                    || (char_bb == 'S' && char_ba == 'M' && char_ab == 'S' && char_aa == 'M')
                    || (char_bb == 'S' && char_ba == 'S' && char_ab == 'M' && char_aa == 'M')
                {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {count}");
}
