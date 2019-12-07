use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let wires: Vec<_> = input("3.txt")?.iter().map(|s| cells(parse(s))).collect();

    println!(
        "Closest = {:?}",
        wires[0]
            .iter()
            .filter_map(|(cell, steps)| wires[1].get(cell).map(|s| steps + s))
            .min()
    );
    Ok(())
}

fn parse(directions: &str) -> Vec<(char, i32)> {
    directions
        .split(",")
        .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap()))
        .collect()
}

fn cells(directions: Vec<(char, i32)>) -> HashMap<(i32, i32), i32> {
    let mut cells = HashMap::new();
    let mut steps = 0;
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in directions {
        for _ in 0..distance {
            match direction {
                'L' => x -= 1,
                'R' => x += 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => panic!("Unknown direction {}", direction),
            }

            steps += 1;
            cells.insert((x, y), steps);
        }
    }

    cells
}
