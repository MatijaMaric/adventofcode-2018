use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn part1(contents: String) -> isize {
    let mut doubles = 0;
    let mut triples = 0;

    for line in contents.lines() {
        let mut counts = HashMap::new();
        for character in line.chars() {
            *counts.entry(character).or_insert(0) += 1;
        }
        if counts.values().any(|&count| count == 2) {
            doubles += 1;
        }
        if counts.values().any(|&count| count == 3) {
            triples += 1;
        }
    }

    return doubles * triples;
}

fn part2(contents: String) -> String {
    for (idx, line) in contents.lines().enumerate() {
        for other in contents.lines().skip(idx + 1) {
            let ans: String = line.chars()
                .zip(other.chars())
                .filter_map(|x| if x.0 == x.1 { Some(x.0) } else { None })
                .collect();
            if ans.len() == line.len() - 1 {
                return ans;
            }
        }
    }
    unreachable!();
}

fn main() {
    let mut file = File::open("day02.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}
