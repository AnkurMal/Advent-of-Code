use std::collections::HashMap;

const DATA: &str = include_str!("../../data/year2024/day_1.txt");

pub fn part_1() {
    let mut list1 = vec![];
    let mut list2 = vec![];
    
    for data in DATA.lines() {
        let mut spl = data.split_whitespace();
        list1.push(spl.next().unwrap().parse::<i64>().unwrap());
        list2.push(spl.next_back().unwrap().parse::<i64>().unwrap());
    }
    
    list1.sort();
    list2.sort();
    list1 = list1.iter().zip(list2.iter()).map(|(x, y)| (x-y).abs())
        .collect::<Vec<i64>>();
    
    println!("Part 1: {}", list1.iter().sum::<i64>())
}

pub fn part_2() {
    let mut list1 = vec![];
    let mut list2 = vec![];

    let mut freq = HashMap::new();
    let mut sum = 0;
    
    for data in DATA.lines() {
        let mut spl = data.split_whitespace();
        list1.push(spl.next().unwrap().parse::<i64>().unwrap());
        list2.push(spl.next_back().unwrap().parse::<i64>().unwrap());
    }

    for i in list2.iter() {
        *freq.entry(i).or_insert(0) += 1
    }

    for i in list1.iter() {
        sum += i*freq.get(i).unwrap_or(&0);
    }
    
    println!("Part 2: {}", sum)
}