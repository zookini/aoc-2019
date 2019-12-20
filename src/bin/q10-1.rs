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

    let best = asteroids.iter().map(|&p| angles(p, &asteroids).len()).max();

    Ok(println!("{:?}", best))
}

type Point = (i8, i8);

fn angles(from: Point, asteroids: &[Point]) -> BTreeSet<i16> {
    asteroids
        .iter()
        .filter(|&&to| from != to)
        .map(|&to| (angle(from, to) * 10.0) as i16)
        .collect()
}

fn angle(from: Point, to: Point) -> f64 {
    let relative = (from.0 - to.0, from.1 - to.1);
    (relative.1 as f64).atan2(relative.0 as f64).to_degrees()
}
