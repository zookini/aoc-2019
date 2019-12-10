use aoc::*;
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let input = input("10.txt")?;
    let map: Vec<_> = input.lines().map(&str::as_bytes).collect();
    let start = (8, 16); // From q10-1
    let mut angles = angles(&map, start.0, start.1);

    for asteroids in angles.values_mut() {
        asteroids.sort_by_key(|(x, y)| {
            -((*x as isize - start.0 as isize).abs() + (*y as isize - start.1 as isize).abs())
        });
    }

    for (i, asteroid) in angles.values_mut().filter_map(&Vec::pop).enumerate() {
        println!("{} {:?}", i + 1, asteroid);
    }

    Ok(())
}

fn angles(map: &[&[u8]], x: usize, y: usize) -> BTreeMap<isize, Vec<(usize, usize)>> {
    let mut angles: BTreeMap<isize, Vec<(usize, usize)>> = BTreeMap::new();

    for y2 in 0..map.len() {
        for x2 in 0..map[y].len() {
            if (y == y2 && x == x2) || (map[y2][x2] == b'.') {
                continue;
            }

            angles
                .entry((angle(x as isize - x2 as isize, y as isize - y2 as isize) * 10.0) as isize)
                .and_modify(|v| v.push((x2, y2)))
                .or_insert(vec![(x2, y2)]);
        }
    }

    angles
}

fn angle(x: isize, y: isize) -> f64 {
    let degrees = (y as f64).atan2(x as f64).to_degrees() - 90.0;
    if degrees >= 0.0 {
        degrees
    } else {
        degrees + 360.0
    }
}
