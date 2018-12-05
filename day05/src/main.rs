extern crate regex;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

fn part1(contents: String) -> usize {
    let mut polymer = contents.clone().trim().to_string();
    let mut shrinked: bool = false;
    let regex = Regex::new(r"(aA)|(Aa)|(bB)|(Bb)|(cC)|(Cc)|(dD)|(Dd)|(eE)|(Ee)|(fF)|(Ff)|(gG)|(Gg)|(hH)|(Hh)|(iI)|(Ii)|(jJ)|(Jj)|(kK)|(Kk)|(lL)|(Ll)|(mM)|(Mm)|(nN)|(Nn)|(oO)|(Oo)|(pP)|(Pp)|(qQ)|(Qq)|(rR)|(Rr)|(sS)|(Ss)|(tT)|(Tt)|(uU)|(Uu)|(vV)|(Vv)|(wW)|(Ww)|(xX)|(Xx)|(yY)|(Yy)|(zZ)|(Zz)").unwrap();
    loop {
        let mut newPolymer = regex.replace_all(&polymer, "").to_string();
        if newPolymer.len() == polymer.len() {
            polymer = newPolymer.clone();
            break;
        }
        polymer = newPolymer.clone();
    }
    return polymer.len();
}

fn part2(contents: String) -> usize {
    let mut min = 10000000000;
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut improved = contents
            .clone()
            .replace(c, "")
            .replace(c.to_uppercase().next().unwrap(), "")
            .to_string();
        let partoneize = part1(improved);
        if partoneize < min {
            min = partoneize;
        }
    }
    return min;
}

fn main() {
    let mut file = File::open("day05.in").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");
    println!("{}", part1(contents.clone()));
    println!("{}", part2(contents.clone()));
}
