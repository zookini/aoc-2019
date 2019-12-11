use aoc::*;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let asteroids: Vec<_> = input("10.txt")?
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| (x as i8, y as i8))
        })
        .collect();

    let best = asteroids
        .iter()
        .map(|&p| (p, angles(&asteroids, p).len()))
        .max_by_key(|(_, visible)| *visible);

    Ok(println!("{:?}", best))
}

type Point = (i8, i8);

fn angles(asteroids: &[Point], (x, y): Point) -> BTreeSet<i16> {
    asteroids
        .iter()
        .filter(|&&to| (x, y) != to)
        .map(|&(x2, y2)| (angle((x - x2, y - y2)) * 10.0) as i16)
        .collect()
}

fn angle((x, y): Point) -> f64 {
    (y as f64).atan2(x as f64).to_degrees()
}
