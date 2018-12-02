use std::fs::File;
use std::io::prelude::*;

fn part_one(contents: String) {
    let mut floor = 0;
    for c in contents.chars() {
        floor += match c {
            ')' => -1,
            '(' => 1,
            _ => 0,
        };
    }
    println!("Part 1: {}", floor);
}

fn part_two(contents: String) {
    let mut floor = 0;
    for (i, c) in contents.chars().enumerate() {
        floor += match c {
            ')' => -1,
            '(' => 1,
            _ => 0,
        };
        if floor < 0 {
            let position = i + 1;
            println!("Part 2: {}", position);
            break;
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("No file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("No string.");

    part_one(contents.clone());
    part_two(contents.clone());
}
