use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("In file {}", "day00.in");

    let mut f = File::open("day00.in").expect("err");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("err");

    println!("Text {}", contents);
}
