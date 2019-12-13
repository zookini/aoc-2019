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
                .map(move |(x, _)| (x as i8, y as i8))
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

type Point = (i8, i8);

fn angles(asteroids: &[Point], from: Point) -> BTreeMap<u16, Vec<Point>> {
    let mut angles: BTreeMap<u16, Vec<Point>> = BTreeMap::new();

    for &to in asteroids {
        if from != to {
            angles
                .entry((angle((to.0 - from.0, to.1 - from.1)) * 10.0) as u16)
                .and_modify(|v| v.push(to))
                .or_insert_with(|| vec![to]);
        }
    }

    angles
}

fn angle((x, y): Point) -> f64 {
    ((y as f64).atan2(x as f64).to_degrees() + 90.0 + 360.0) % 360.0
}
