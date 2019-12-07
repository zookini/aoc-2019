use aoc::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let wires: Vec<_> = input("3.txt")?.iter().map(|s| cells(parse(s))).collect();
    let crossed = wires[0].intersection(&wires[1]);

    println!(
        "Closest = {:?}",
        crossed.map(|(x, y)| x.abs() + y.abs()).min()
    );
    Ok(())
}

fn parse(directions: &str) -> Vec<(char, i32)> {
    directions
        .split(",")
        .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap()))
        .collect()
}

fn cells(directions: Vec<(char, i32)>) -> HashSet<(i32, i32)> {
    let mut cells = HashSet::new();
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

            cells.insert((x, y));
        }
    }

    cells
}
