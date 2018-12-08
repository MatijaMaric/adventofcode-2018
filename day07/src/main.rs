extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn solution(contents: String) {
    let mut graph: HashMap<char, HashSet<char>> = HashMap::new();

    for line in contents.lines() {
        let from = line.as_bytes()[5] as char;
        let to = line.as_bytes()[36] as char;

        graph.entry(to).or_insert_with(HashSet::new).insert(from);
        graph.entry(from).or_insert_with(HashSet::new);
    }

    part1(&graph);
    part2(&graph);
}

fn part1(graph: &HashMap<char, HashSet<char>>) {
    let mut taken: HashSet<char> = HashSet::new();
    let mut res: Vec<char> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    loop {
        for (&c, req) in graph {
            if !taken.contains(&c) {
                if req.iter().all(|s| taken.contains(s)) {
                    stack.push(c);
                }
            }
        }
        stack.sort();
        stack.dedup();
        stack.reverse();
        let next = stack.pop();
        if next.is_some() {
            let next = next.unwrap();
            taken.insert(next);
            res.push(next);
        }
        if next.is_none() {
            break;
        }
    }

    println!("{:?}", res.into_iter().collect::<String>());
}

fn part2(graph: &HashMap<char, HashSet<char>>) {
    let mut taken: HashSet<char> = HashSet::new();
    let mut stack: Vec<char> = Vec::new();
    let mut busy: Vec<(char, usize)> = Vec::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut seconds = 0;

    loop {
        for (&c, req) in graph {
            if !taken.contains(&c) {
                if req.iter().all(|s| done.contains(s)) {
                    stack.push(c);
                }
            }
        }
        stack.sort();
        stack.dedup();
        stack.reverse();

        for d in busy.clone().into_iter() {
            if d.1 == 1 {
                done.insert(d.0);
            }
        }
        busy = busy.into_iter()
            .map(|x| (x.0, x.1 - 1))
            .filter(|&x| x.1 > 0)
            .collect();

        if busy.len() < 5 {
            loop {
                let next = stack.pop();
                if next.is_some() {
                    let next = next.unwrap();
                    taken.insert(next);
                    busy.push((next, 60 + (((next as u8) - ('A' as u8)) as usize)));
                }
                if next.is_none() || busy.len() == 5 {
                    break;
                }
            }
        }

        seconds += 1;
        if done.len() == graph.keys().len() {
            break;
        }
    }

    println!("{}", seconds);
}

fn main() {
    let mut file = File::open("./day07.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    solution(contents.clone())
}
