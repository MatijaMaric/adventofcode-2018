extern crate pbr;
extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use pbr::ProgressBar;

fn solution(contents: String) {
    let r = Regex::new("([0-9]*) players; last marble is worth ([0-9]*) points").unwrap();
    let cap = r.captures_iter(&contents).next().unwrap();
    let players = cap.get(1) // I don't care
        .map(|x| x.as_str())
        .expect("err")
        .parse::<usize>()
        .expect("err");
    let last_marble = cap.get(2)
        .map(|x| x.as_str())
        .expect("err")
        .parse::<usize>()
        .expect("err") * 100;

    let mut current_player = 0;
    let mut current_marble = 1;

    let mut score: HashMap<usize, usize> = HashMap::new();
    let mut circle: Vec<usize> = Vec::new();

    let mut pos = 0;
    circle.push(0);

    let mut pb = ProgressBar::new(last_marble as u64);

    while current_marble <= last_marble {
        if current_marble % 23 == 0 {
            *score.entry(current_player).or_insert(0) += current_marble;
            if pos < 7 {
                pos = circle.len() - (7 - pos);
            } else {
                pos -= 7;
            }
            *score.entry(current_player).or_insert(0) += circle[pos];
            circle.remove(pos);
        } else {
            pos += 2;
            if pos > circle.len() {
                pos -= circle.len();
            }
            circle.insert(pos, current_marble);
        }
        //println!("{:?}", circle);
        current_player = (current_player + 1) % players;
        current_marble += 1;
        pb.set(current_marble as u64);
    }
    let max_score = score.values().max().unwrap();
    println!("{:?}", max_score);
}

fn main() {
    let mut file = File::open("day09.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    solution(contents.clone());
}
