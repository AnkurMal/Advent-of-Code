use anyhow::Result;

const DATA: &str = include_str!("../../data/year2021/day_3.txt");

macro_rules! operate_str {
    ($st: expr, $sym: tt) => {
        for i in 0..$st[0].len() {
            let (mut count0, mut count1) = (0, 0);
            for val in &$st {
                let bytes = val.as_bytes();
                if (bytes[i] == b'0') {
                    count0 += 1;
                } else {
                    count1 += 1;
                }
            }

            let mut res1 = vec![];
            if (count1 $sym count0) {
                for val in &$st {
                    let bytes = val.as_bytes();
                    if bytes[i] == b'1' {
                        res1.push(*val);
                    }
                }
            } else {
                for val in &$st {
                    let bytes = val.as_bytes();
                    if bytes[i] == b'0' {
                        res1.push(*val);
                    }
                }
            }

            $st = res1;
            if ($st.len() == 1) {
                break;
            }
        }
    };
}

pub fn part_1() -> Result<()> {
    let lines = DATA.lines().collect::<Vec<&str>>();
    let mut vec = vec![String::new(); lines[0].len()];

    for data in &lines {
        for (i, ch) in data.chars().enumerate() {
            vec[i].push(ch);
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in &vec {
        let count0 = i.matches('0').count();
        let count1 = i.len() - count0;

        if (count1 > count0) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!(
        "Part 1: {:?}",
        i32::from_str_radix(&gamma, 2)? * i32::from_str_radix(&epsilon, 2)?
    );

    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut st = DATA.lines().collect::<Vec<&str>>();
    let mut st1 = st.clone();

    operate_str!(st, >=);
    operate_str!(st1, <);

    println!(
        "Part 2: {:?}",
        i32::from_str_radix(st[0], 2)? * i32::from_str_radix(st1[0], 2)?
    );

    Ok(())
}
