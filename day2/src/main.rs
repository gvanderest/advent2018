use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines = contents.trim().split("\n");

    let mut two_count = 0;
    let mut three_count = 0;

    for line in lines.clone() {
        let mut character_counts: HashMap<char, u32> = HashMap::new();
        for character in line.chars() {
            let character_count = match character_counts.get(&character) {
                Some(existing_count) => existing_count + 1,
                None => 1,
            };
            character_counts.insert(character, character_count);
        }

        let mut two_found = false;
        let mut three_found = false;
        for (_, count) in character_counts.iter() {
            if *count == 2 {
                two_found = true;
            }
            if *count == 3 {
                three_found = true;
            }
        }

        if two_found {
            two_count += 1;
        }

        if three_found {
            three_count += 1;
        }
    }

    println!("Part 1: {}", two_count * three_count);

    'outer: for first_line in lines.clone() {
        'inner: for second_line in lines.clone() {
            let mut diff_count = 0;
            let mut matching_id = String::new();
            for (index, first_char) in first_line.chars().enumerate() {
                let second_char = second_line.chars().nth(index).unwrap();

                if first_char != second_char {
                    diff_count += 1;
                } else {
                    matching_id.push(first_char)
                }
            }

            if diff_count == 1 {
                println!("Part 2: {}", matching_id);
                break 'outer;
            }
        }
    }
    Ok(())
}
