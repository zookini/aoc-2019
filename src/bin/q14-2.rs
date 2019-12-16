use aoc::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = input("14.txt")?;

    let reactions: HashMap<&str, Reaction> = input
        .lines()
        .map(&Reaction::parse)
        .map(|r| (r.output.name, r))
        .collect();

    let mut low = 0;
    let mut high = 10u64.pow(12);

    while low < high - 1 {
        let mut state = HashMap::new();
        let fuel = (low + high) / 2;

        cost(&reactions, &mut state, &Chemical::new("FUEL", fuel));

        match state["ORE"].cmp(&10u64.pow(12)) {
            Ordering::Greater => high = fuel,
            _ => low = fuel,
        }
    }

    Ok(println!("{}", low))
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

        for i in &reactions[chemical.name].inputs {
            cost(reactions, state, &Chemical::new(i.name, n * i.units));
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
