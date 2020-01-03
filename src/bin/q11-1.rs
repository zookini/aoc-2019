use aoc::*;
use futures::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let (mut tx, rx, _) = Computer::load("11.txt")?.spawn();
    let mut moves = rx.chunks(2).map(|v| (v[0], v[1]));

    let mut position = (0, 0);
    let mut facing = 0;
    let mut painted = HashMap::new();

    loop {
        tx.send(*painted.get(&position).unwrap_or(&0)).await?;

        if let Some((colour, direction)) = moves.next().await {
            painted.insert(position, colour);
            facing = turn(facing, direction);
            position = step(position, facing);
        } else {
            break;
        }
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
