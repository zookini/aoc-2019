use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = input("6.txt")?;

    let universe = input
        .lines()
        .map(|line| line.split(')').collect_tuple().unwrap())
        .into_group_map();

    Ok(println!("{}", calculate(&universe, "COM", 0)))
}

fn calculate(universe: &HashMap<&str, Vec<&str>>, object: &str, distance: u32) -> u32 {
    universe
        .get(object)
        .map(|v| v.iter().map(|o| calculate(universe, o, distance + 1)).sum())
        .unwrap_or(0)
        + distance
}
