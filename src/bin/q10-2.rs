use aoc::*;
use std::collections::BTreeMap;

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

    let start = (8, 16); // From q10-1
    let mut angles = angles(&asteroids, start);

    for targets in angles.values_mut() {
        targets.sort_by_key(|&(x, y)| -((x - start.0).abs() + (y - start.1).abs()));
    }

    for (i, target) in angles.values_mut().filter_map(&Vec::pop).enumerate() {
        println!("{} {:?}", i + 1, target);
    }

    Ok(())
}

type Point = (isize, isize);

fn angles(asteroids: &[Point], (x, y): Point) -> BTreeMap<isize, Vec<Point>> {
    let mut angles: BTreeMap<isize, Vec<Point>> = BTreeMap::new();

    for &(x2, y2) in asteroids {
        if !(y == y2 && x == x2) {
            angles
                .entry((angle((x2 - x, y2 - y)) * 10.0) as isize)
                .and_modify(|v| v.push((x2, y2)))
                .or_insert(vec![(x2, y2)]);
        }
    }

    angles
}

fn angle((x, y): Point) -> f64 {
    ((y as f64).atan2(x as f64).to_degrees() + 90.0 + 360.0) % 360.0
}
