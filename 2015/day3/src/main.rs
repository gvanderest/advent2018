use std::fs::File;
use std::io::prelude::*;

fn get_direction_coords_modifier(direction: char) -> (i8, i8) {
    match direction {
        'v' => (0, -1),
        '^' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid direction: {}", direction),
    }
}

fn count_houses(directions: &String) -> usize {
    let mut coords = (0, 0);
    let mut visited_houses = vec![coords];

    for dir in directions.chars() {
        let (x, y) = coords;
        let (mx, my) = get_direction_coords_modifier(dir);
        coords = (x + mx, y + my);
        visited_houses.push(coords);
    }

    visited_houses.sort();
    visited_houses.dedup();
    visited_houses.len()
}

fn count_robo_santa_houses(directions: &String) -> usize {
    let mut coords = [(0, 0), (0, 0)];
    let mut visited_houses = vec![(0, 0)];
    let mut robo_turn = false;

    for dir in directions.chars() {
        let santa_index = match robo_turn {
            false => 0,
            true => 1,
        };
        let (x, y) = coords[santa_index];
        let (mx, my) = get_direction_coords_modifier(dir);
        let new_coords = (x + mx, y + my);
        coords[santa_index] = new_coords;

        visited_houses.push(new_coords);

        robo_turn = !robo_turn;
    }

    visited_houses.sort();
    visited_houses.dedup();
    visited_houses.len()
}

fn main() {
    let mut file = File::open("input.txt").expect("Couldn't open file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file.");

    assert_eq!(count_houses(&">".to_string()), 2);
    assert_eq!(count_houses(&"^>v<".to_string()), 4);
    assert_eq!(count_houses(&"^v^v^v^v^v".to_string()), 2);

    let house_count = count_houses(&contents);
    println!("First part: {}", house_count);

    assert_eq!(count_robo_santa_houses(&"^v".to_string()), 3);
    assert_eq!(count_robo_santa_houses(&"^>v<".to_string()), 3);
    assert_eq!(count_robo_santa_houses(&"^v^v^v^v^v".to_string()), 11);

    let house_count = count_robo_santa_houses(&contents);
    println!("Second part: {}", house_count);
}
