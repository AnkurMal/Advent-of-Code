const DATA: &str = include_str!("../../data/year2015/day_5.txt");

pub fn part_1() {
    let mut total = 0;

    'outer: for str in DATA.lines() {
        let mut vowel = 0;
        let mut twice = 0;

        if !(str.contains("ab") || str.contains("cd") 
        || str.contains("pq") || str.contains("xy"))
        {            
            for (i, ch) in str.chars().enumerate() {
                if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
                    vowel += 1;
                }
                
                if let Some(next) = str.chars().nth(i+1)
                    && ch==next {
                        twice += 1;
                    }

                if vowel >= 3 && twice >= 1 {
                    total += 1;
                    continue 'outer;
                }
            }
        }  
    }

    println!("Part 1: {total}");
}

pub fn part_2() {
    let mut total = 0;

    'outer: for str in DATA.lines() {
        let mut twice = false;
        let mut repeat = false;

        for i in 0..str.len()-1 {
            let chunk = &str[i..i+2];
            if let Some(next) = &str[i+2..].find(chunk) {
                twice = true;
                break;
            }
        }

        for (i, ch) in str.chars().enumerate() {
            if let Some(next) = str.chars().nth(i+2)
                && ch==next {
                    repeat = true;
                    break;
                }
        }

        if twice && repeat {
            total += 1;
        }
    }

    println!("Part 2: {total}");
}