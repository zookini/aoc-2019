use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("11.txt")?.spawn();

    let mut position = (0, 0);
    let mut facing = 0;
    let mut canvas = vec![vec![0; 50]; 6];

    canvas[0][0] = 1;
    tx.send(canvas[position.1 as usize][position.0 as usize])?;

    for (colour, direction) in rx.iter().tuples() {
        canvas[position.1 as usize][position.0 as usize] = colour;
        facing = turn(facing, direction);
        position = step(position, facing);

        tx.send(canvas[position.1 as usize][position.0 as usize])?;
    }

    for row in canvas {
        let line = row
            .iter()
            .map(|&i| if i == 1 { '#' } else { ' ' })
            .collect::<String>();

        println!("{}", line);
    }

    Ok(())
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
