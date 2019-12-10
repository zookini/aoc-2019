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
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    let best = asteroids
        .iter()
        .map(|&p| (p, angles(&asteroids, p).len()))
        .max_by_key(|(_, visible)| *visible);

    Ok(println!("{:?}", best))
}

type Point = (isize, isize);

fn angles(asteroids: &[Point], (x, y): Point) -> BTreeSet<isize> {
    asteroids
        .iter()
        .filter(|&&(x2, y2)| !(x == x2 && y == y2))
        .map(|&(x2, y2)| (angle((x - x2, y - y2)) * 10.0) as isize)
        .collect()
}

fn angle((x, y): Point) -> f64 {
    (y as f64).atan2(x as f64).to_degrees()
}
