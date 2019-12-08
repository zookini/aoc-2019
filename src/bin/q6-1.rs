use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = input("6.txt")?;
    let mut universe: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let orbit: Vec<&str> = line.split(')').collect();

        universe
            .entry(orbit[0])
            .and_modify(|v| v.push(orbit[1]))
            .or_insert(vec![orbit[1]]);
    }

    Ok(println!("{}", calculate(&universe, "COM", 0)))
}

fn calculate(universe: &HashMap<&str, Vec<&str>>, object: &str, distance: u32) -> u32 {
    universe
        .get(object)
        .map(|v| v.iter().map(|o| calculate(universe, o, distance + 1)).sum())
        .unwrap_or(0)
        + distance
}
