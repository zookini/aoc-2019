use aoc::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = input("14.txt")?;

    let reactions: HashMap<&str, Reaction> = input
        .lines()
        .map(&Reaction::parse)
        .map(|r| (r.output.name, r))
        .collect();

    let mut state = HashMap::new();

    cost(&reactions, &mut state, &Chemical::new("FUEL", 1));
    Ok(println!("{}", state["ORE"]))
}

fn cost(
    reactions: &HashMap<&str, Reaction>,
    state: &mut HashMap<String, u64>,
    chemical: &Chemical,
) {
    let count = state.entry(chemical.name.into()).or_insert(0);

    if chemical.name == "ORE" {
        *count += chemical.units;
    } else if chemical.units < *count {
        *count -= chemical.units;
    } else {
        let min = reactions[chemical.name].output.units;
        let missing = chemical.units - *count;
        let n = missing / min + if missing % min == 0 { 0 } else { 1 };

        *count += n * min;
        *count -= chemical.units;

        for input in &reactions[chemical.name].inputs {
            cost(
                reactions,
                state,
                &Chemical::new(input.name, n * input.units),
            );
        }
    }
}

#[derive(Debug)]
struct Reaction<'a> {
    inputs: Vec<Chemical<'a>>,
    output: Chemical<'a>,
}

impl<'a> Reaction<'a> {
    fn parse(s: &'a str) -> Self {
        let (inputs, output) = s.split(" => ").collect_tuple().unwrap();

        Reaction {
            inputs: inputs.split(", ").map(&Chemical::parse).collect(),
            output: Chemical::parse(output),
        }
    }
}

#[derive(Debug)]
struct Chemical<'a> {
    name: &'a str,
    units: u64,
}

impl<'a> Chemical<'a> {
    fn parse(s: &'a str) -> Self {
        let (units, name) = s.split(' ').collect_tuple().unwrap();
        Self::new(name, units.parse().unwrap())
    }

    fn new(name: &'a str, units: u64) -> Self {
        Chemical { name, units }
    }
}
