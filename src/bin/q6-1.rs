use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut universe: HashMap<String, Vec<String>> = HashMap::new();

    for line in input("6.txt")? {
        let orbit: Vec<&str> = line.split(')').collect();

        universe
            .entry(orbit[0].into())
            .and_modify(|v| v.push(orbit[1].into()))
            .or_insert(vec![orbit[1].into()]);
    }

    Ok(println!("{}", calculate(&universe, "COM", 0)))
}

fn calculate(universe: &HashMap<String, Vec<String>>, object: &str, distance: u32) -> u32 {
    universe
        .get(object)
        .map(|v| v.iter().map(|o| calculate(universe, o, distance + 1)).sum())
        .unwrap_or(0)
        + distance
}
