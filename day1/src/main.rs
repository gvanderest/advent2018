use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines = contents.trim().split("\n");
    let mut total = 0;
    for line in lines {
        let symbol = &line[0..1];
        let value = &line[1..].parse::<i32>().unwrap() * if symbol == "+" { 1 } else { -1 };
        total += value;
    }

    println!("Part 1: {}", total);

    let mut seen_values = Vec::new();
    let mut total = 0;

    'outer: loop {
        let new_lines = contents.trim().split("\n");
        for line in new_lines {
            let symbol = &line[0..1];
            let value = &line[1..].parse::<i32>().unwrap() * if symbol == "+" { 1 } else { -1 };
            total = total + value;
            if seen_values.contains(&total) {
                println!("Part 2: {}", total);
                break 'outer;
            } else {
                seen_values.push(total);
            }
        }
    }
    Ok(())
}
