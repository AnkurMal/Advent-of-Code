const DATA: &str = include_str!("../../data/year2024/day_5.txt");

pub fn part_1() {
    let mut sum = 0;

    let spl = DATA.lines().collect::<Vec<&str>>();
    let empty = spl.iter().position(|&x| x.is_empty()).unwrap();

    let (pages, list) = spl.split_at(empty);

    'outer: for &i in list.iter().skip(1) {
        let st = i.split(',').collect::<Vec<&str>>();

        for j in 0..st.len() - 1 {
            let page = format!("{}|{}", st[j], st[j + 1]);
            if !pages.contains(&page.as_str()) {
                continue 'outer;
            }
        }

        sum += st[(st.len() - 1) / 2].parse::<i32>().unwrap();
    }

    println!("Part 1: {}", sum);
}

pub fn part_2() {
    let mut sum = 0;

    let spl = DATA.lines().collect::<Vec<&str>>();
    let empty = spl.iter().position(|&x| x.is_empty()).unwrap();

    let (pages, list) = spl.split_at(empty);

    for &i in list.iter().skip(1) {
        let mut st = i.split(',').collect::<Vec<&str>>();
        let mut contains = true;
        let mut ind = 0;

        while ind < st.len() - 1 {
            let page = format!("{}|{}", st[ind], st[ind + 1]);

            if !pages.contains(&page.as_str()) {
                st.swap(ind, ind + 1);
                ind = 0;
                contains = false;
                continue;
            }
            ind += 1;
        }

        if !contains {
            sum += st[(st.len() - 1) / 2].parse::<i32>().unwrap();
        }
    }

    println!("Part 2: {}", sum);
}
