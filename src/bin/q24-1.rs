use aoc::*;
use itertools::iproduct;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = input("24.txt")?;
    let mut seen = HashSet::new();

    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    while !seen.contains(&grid) {
        let previous = grid;
        grid = step(&previous);
        seen.insert(previous);
    }

    Ok(println!("{}", biodiversity(&grid)))
}

fn step(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut update = grid.clone();

    for (x, y) in iproduct!(0..grid[0].len(), 0..grid.len()) {
        let bugs = neighbours(&grid, x as i8, y as i8);

        if grid[y][x] == b'#' && bugs != 1 {
            update[y][x] = b'.';
        } else if grid[y][x] == b'.' && [1, 2].contains(&bugs) {
            update[y][x] = b'#';
        }
    }

    update
}

fn biodiversity(grid: &Vec<Vec<u8>>) -> usize {
    grid.iter().flatten().enumerate().fold(0, |bd, (i, b)| {
        bd + if *b == b'#' { 2usize.pow(i as u32) } else { 0 }
    })
}

fn neighbours(grid: &Vec<Vec<u8>>, x: i8, y: i8) -> u8 {
    [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
        .iter()
        .filter(|&&(x, y)| x >= 0 && y >= 0 && x < grid[0].len() as i8 && y < grid.len() as i8)
        .filter(|&&(x, y)| grid[y as usize][x as usize] == b'#')
        .count() as u8
}
