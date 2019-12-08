use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let wires: Vec<_> = input("3.txt")?.lines().map(|s| cells(parse(s))).collect();

    println!(
        "{:?}",
        wires[0]
            .iter()
            .filter_map(|(cell, steps)| wires[1].get(cell).map(|s| steps + s))
            .min()
    );
    Ok(())
}

fn parse<'a>(directions: &'a str) -> impl Iterator<Item = (char, i32)> + 'a {
    directions
        .split(",")
        .map(|s| (s.chars().nth(0).unwrap(), s[1..].parse().unwrap()))
}

fn cells(directions: impl IntoIterator<Item = (char, i32)>) -> HashMap<(i32, i32), i32> {
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
                _ => unreachable!(),
            }

            steps += 1;
            cells.insert((x, y), steps);
        }
    }

    cells
}
