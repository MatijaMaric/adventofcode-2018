use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

struct Claim {
    id: usize,
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

fn parseClaims(contents: String) -> Vec<Claim> {
    return contents
        .lines()
        .map(|line| {
            let mut num = line.split(|c| {
                c == '#' || c == ' ' || c == '@' || c == ',' || c == ':' || c == 'x'
            }).filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            return Claim {
                id: num[0],
                top: num[1],
                left: num[2],
                height: num[3],
                width: num[4],
            };
        })
        .collect();
}

fn solution(contents: String) {
    let claims: Vec<Claim> = parseClaims(contents);
    let mut fabric = HashMap::new();
    let mut fabric_claim = HashMap::new();
    let mut all = HashSet::new();
    let mut intersects = HashSet::new();

    for claim in claims {
        all.insert(claim.id);
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                *fabric.entry((i, j)).or_insert(0) += 1;
                if fabric_claim.contains_key(&(i, j)) {
                    intersects.insert(claim.id);
                    intersects.insert(fabric_claim[&(i, j)]);
                } else {
                    fabric_claim.insert((i, j), claim.id);
                }
            }
        }
    }

    let part1 = fabric.values().filter(|x| **x > 1).count();
    let part2 = all.difference(&intersects).next();

    println!("{:?}", part1);
    println!("{:?}", part2);
}

fn main() {
    let mut file = File::open("day03.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    solution(contents.clone())
}
