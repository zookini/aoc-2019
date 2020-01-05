use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("11.txt")?.spawn();

    let mut position = (0, 0);
    let mut facing = 0;
    let mut painted = HashMap::new();

    tx.send(*painted.get(&position).unwrap_or(&0))?;

    for (colour, direction) in rx.iter().tuples() {
        painted.insert(position, colour);
        facing = turn(facing, direction);
        position = step(position, facing);

        tx.send(*painted.get(&position).unwrap_or(&0))?;
    }

    Ok(println!("{}", painted.len()))
}

type Point = (i8, i8);

fn turn(facing: u16, input: i64) -> u16 {
    (facing + if input == 0 { 270 } else { 90 }) % 360
}

fn step((x, y): Point, direction: u16) -> Point {
    match direction {
        0 => (x, y - 1),
        90 => (x + 1, y),
        180 => (x, y + 1),
        270 => (x - 1, y),
        _ => unreachable!(),
    }
}
