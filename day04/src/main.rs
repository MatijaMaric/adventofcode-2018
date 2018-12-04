use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn solution(contents: String) {
    let mut lines: Vec<&str> = contents.lines().collect::<Vec<&str>>();
    lines.sort();
    let mut guard_id: usize = 0;
    let mut asleep: usize = 0;
    let mut tab = HashMap::new();
    let mut total = HashMap::new();
    for line in lines {
        let mut r: Vec<&str> = line.split(|c| c == '[' || c == ']')
            .map(|p| p.trim())
            .collect();
        let mut q: Vec<&str> = r[1].split(':').collect();
        let time = q[1].parse::<usize>().unwrap();
        if r[2].starts_with("Guard") {
            let mut s: Vec<&str> = r[2].split(|c| c == '#' || c == ' ').collect();
            guard_id = s[2].parse::<usize>().unwrap();
            asleep = 0;
        }
        if r[2].starts_with("falls asleep") {
            asleep = time;
        }
        if r[2].starts_with("wakes up") {
            for t in asleep..(time + 1) {
                *tab.entry((guard_id, t)).or_insert(0) += 1;
                *total.entry(guard_id).or_insert(0) += 1;
            }
        }
    }
    let mut guard = guard_id;
    for g in total.keys() {
        //println!("#{} {}", g, total[g]);
        if total[&g] > total[&guard] {
            guard = *g;
        }
    }

    let mut guard2 = guard_id;
    let mut minute2 = 0;
    let mut max_minute = 0;

    // Multiple solutions (╯°□°）╯︵ ┻━┻
    let mut minute = 0;
    for i in 0..60 {
        //println!("#{} - {} {}", guard, i, tab[&(guard, i)]);
        if tab[&(guard, i)] > tab[&(guard, minute)] {
            minute = i;
        }
        for g in total.keys() {
            if tab.contains_key(&(*g, i)) {
                if tab[&(*g, i)] > max_minute {
                    guard2 = *g;
                    minute2 = i;
                    max_minute = tab[&(*g, i)];
                }
            }
        }
    }

    println!("{} {}", guard, minute);
    println!("{}", guard * minute);

    println!("{} {}", guard2, minute2);
    println!("{}", guard2 * minute2);
}

fn main() {
    let mut file = File::open("day04.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    solution(contents.clone());
}
