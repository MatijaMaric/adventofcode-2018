use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

struct Point {
    x: isize,
    y: isize,
}

fn solution(contents: String) {
    let mut min_x = 1000000;
    let mut min_y = 1000000;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut outers = HashSet::new();
    let mut counts = HashMap::new();
    let points: Vec<Point> = contents
        .lines()
        .map(|line| {
            let r: Vec<isize> = line.split(", ")
                .filter_map(|x| x.parse::<isize>().ok())
                .collect();
            if r[0] > max_x {
                max_x = r[0];
            }
            if r[0] < min_x {
                min_x = r[0];
            }
            if r[1] > max_y {
                max_y = r[1];
            }
            if r[1] < min_y {
                min_y = r[1];
            }
            return Point { x: r[0], y: r[1] };
        })
        .collect();

    let mut part2 = 0;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let mut min_dist = 100000000;
            let mut closest = None;
            let mut dist_to_all = 0;
            for i in 0..points.len() {
                let dist = (points[i].x - x).abs() + (points[i].y - y).abs();
                if dist < min_dist {
                    min_dist = dist;
                    closest = Some(i);
                } else if dist == min_dist {
                    closest = None;
                }
                dist_to_all += dist;
            }
            if dist_to_all < 10000 {
                part2 += 1;
            }
            if closest.is_some() {
                let closest = closest.unwrap();
                *counts.entry(closest).or_insert(0) += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    outers.insert(closest);
                }
            }
        }
    }

    let part1 = counts
        .keys()
        .filter(|key| !outers.contains(key))
        .map(|key| counts[key])
        .max()
        .unwrap();

    println!("{}", part1);

    println!("{}", part2);
}

fn main() {
    let mut file = File::open("day06.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    solution(contents);
}
