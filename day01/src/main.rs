use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn part1(contents: String) -> i32 {
    let freq = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .fold(0, |acc, x| acc + x);
    return freq;
}

fn part2(contents: String) -> i32 {
    let mut freq = 0;
    let mut freqs = HashSet::new();
    freqs.insert(freq);
    loop {
        for f in contents.lines().map(|line| line.parse::<i32>().unwrap()) {
            freq = freq + f;
            if freqs.contains(&freq) {
                return freq;
            }
            freqs.insert(freq);
        }
    }
}

fn main() {
    let mut file = File::open("day01.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}
