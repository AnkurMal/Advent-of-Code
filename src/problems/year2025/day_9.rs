use anyhow::Result;
use geo::{Contains, LineString, Polygon, polygon};
use rayon::prelude::*;

const DATA: &str = include_str!("../../data/year2025/day_9.txt");

pub fn part_1() -> Result<()> {
    let vec = DATA
        .lines()
        .map(|x| {
            x.split(',')
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut area = 0;

    for (i, list) in vec.iter().enumerate() {
        for dim in vec.iter().skip(i + 1) {
            area = area.max(((dim[0] - list[0]).abs() + 1) * ((dim[1] - list[1]).abs() + 1));
        }
    }

    println!("Part 1: {area}");
    Ok(())
}

pub fn part_2() -> Result<()> {
    let mut vec = DATA
        .lines()
        .map(|x| {
            let spl = x
                .split(',')
                .map(|y| y.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            (spl[0], spl[1])
        })
        .collect::<Vec<_>>();
    vec.push(vec[0]);
    let polygon = Polygon::new(LineString::from(vec.clone()), vec![]);

    let area = vec
        .par_iter()
        .enumerate()
        .map(|(i, &(x1, y1))| {
            let mut local_max = 0i64;
            for &(x2, y2) in vec.iter().skip(i + 1) {
                let rec = geo::polygon![
                    (x: x2, y: y2),
                    (x: x1, y: y2),
                    (x: x1, y: y1),
                    (x: x2, y: y1),
                    (x: x2, y: y2)
                ];
                if polygon.contains(&rec) {
                    let current = ((x2 - x1).abs() as i64 + 1) * ((y2 - y1).abs() as i64 + 1);
                    if current > local_max {
                        local_max = current;
                    }
                }
            }
            local_max
        })
        .max()
        .unwrap_or(0);

    println!("Part 2: {}", area);
    Ok(())
}
