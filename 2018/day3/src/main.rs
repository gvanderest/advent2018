use std::fs::File;
use std::io::prelude::*;

/**
 * Example: "#1 @ 1,3: 4x4"
 */
fn split_line(line: &str) -> (String, usize, usize, usize, usize) {
    let mut parts = line.split(' ');

    let id = parts
        .nth(0)
        .expect("Unable to extract ID")
        .trim_start_matches("#")
        .to_string();

    let coords = parts
        .nth(1)
        .expect("Unable to extract coords")
        .trim_end_matches(":");

    let mut coords_parts = coords.split(',');

    let x = coords_parts
        .nth(0)
        .expect("Unable to extract X coord")
        .to_string()
        .parse::<usize>()
        .unwrap();

    let y = coords_parts
        .nth(0)
        .expect("Unable to extract Y coord")
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut size_parts = parts.nth(0).expect("Unable to parse size").split('x');

    let width = size_parts
        .nth(0)
        .expect("Unable to parse width")
        .to_string()
        .parse::<usize>()
        .unwrap();

    let height = size_parts
        .nth(0)
        .expect("Unable to parse height")
        .to_string()
        .parse::<usize>()
        .unwrap();

    (id, x, y, width, height)
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open input file.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read input file.");

    let lines = contents.trim().split("\n");

    const MAX_WIDTH: usize = 1000;
    const MAX_HEIGHT: usize = 1000;

    let mut grid: [[usize; MAX_HEIGHT]; MAX_WIDTH] = [[0; MAX_HEIGHT]; MAX_WIDTH];

    for line in lines.clone() {
        let (_id, x, y, width, height) = split_line(&line);
        for ix in x..x + width {
            for iy in y..y + height {
                grid[ix][iy] += 1;
            }
        }
    }

    let mut count = 0;
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            count += match grid[x][y] {
                d if d > 1 => 1,
                _ => 0,
            }
        }
    }
    println!("Part one: {}", count);

    'line_loop: for line in lines.clone() {
        let (id, x, y, width, height) = split_line(&line);
        for ix in x..x + width {
            for iy in y..y + height {
                if grid[ix][iy] > 1 {
                    continue 'line_loop;
                }
            }
        }
        println!("Part two: {}", id);
    }
}
