use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;

fn calculate_area(l: u32, w: u32, h: u32) -> u32 {
    let result: u32 = 2 * l * w + 2 * w * h + 2 * h * l;

    let mut sides = vec![l, w, h];
    sides.sort();

    let result_with_slack = result + (sides[0] * sides[1]);

    result_with_slack
}

fn calculate_ribbon(l: u32, w: u32, h: u32) -> u32 {
    let mut sides = vec![l, w, h];
    sides.sort();

    let one = sides[0];
    let two = sides[1];

    let perimeter = (one + two) * 2;

    let volume = l * w * h;

    let bow_length = volume + perimeter;

    bow_length
}

fn main() {
    let mut file = File::open("input.txt").expect("Couldn't load file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file.");

    let lines = contents.trim().split("\n");

    let mut total: u32 = 0;
    let mut w = 0;
    let mut l = 0;
    let mut h = 0;
    let mut ribbon_total: u32 = 0;
    for line in lines {
        for (i, part) in line.split("x").enumerate() {
            let value = part.parse::<u32>().unwrap();
            match i {
                0 => l = value,
                1 => w = value,
                2 => h = value,
                _ => panic!("Invalid value: {}", part),
            }
        }
        total += calculate_area(l, w, h);
        ribbon_total += calculate_ribbon(l, w, h);
    }
    println!("Part 1: {}", total);
    println!("Part 2: {}", ribbon_total);
}
