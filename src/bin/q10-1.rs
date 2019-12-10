use aoc::*;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = input("10.txt")?;
    let map: Vec<_> = input.lines().map(&str::as_bytes).collect();
    let mut best = ((0, 0), 0);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'#' {
                let visible = angles(&map, x, y).len();
                if visible > best.1 {
                    best = ((x, y), visible);
                }
            }
        }
    }

    Ok(println!("{:?}", best))
}

fn angles(map: &[&[u8]], x: usize, y: usize) -> BTreeSet<isize> {
    let mut angles: BTreeSet<isize> = BTreeSet::new();

    for y2 in 0..map.len() {
        for x2 in 0..map[y].len() {
            if map[y2][x2] == b'#' && !(y == y2 && x == x2) {
                angles.insert(
                    (angle(x as isize - x2 as isize, y as isize - y2 as isize) * 10.0) as isize,
                );
            }
        }
    }

    angles
}

fn angle(x: isize, y: isize) -> f64 {
    (y as f64).atan2(x as f64).to_degrees()
}
