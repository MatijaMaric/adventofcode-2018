use std::fs::File;
use std::io::prelude::*;

fn solution(contents: String) {
    let mut r = contents
        .split_whitespace()
        .map(|x| x.parse::<usize>().expect("parse"));

    //println!("{:?}", part1(&mut r));
    println!("{:?}", part2(&mut r));
}

fn part1<I>(it: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    let children = it.next().unwrap();
    let metas = it.next().unwrap();
    return (0..children).map(|_| part1(it)).sum::<usize>() + it.take(metas).sum::<usize>();
}

fn part2<I>(it: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    let children = it.next().unwrap();
    let metas = it.next().unwrap();
    if children == 0 {
        return it.take(metas).sum::<usize>();
    } else {
        let child_vals: Vec<usize> = (0..children).map(|_| part2(it)).collect();
        return it.take(metas)
            .filter(|&i| i >= 1 && i <= children)
            .map(|id| child_vals[id - 1])
            .sum::<usize>();
    }
}

fn main() {
    let mut file = File::open("day08.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    solution(contents);
}
