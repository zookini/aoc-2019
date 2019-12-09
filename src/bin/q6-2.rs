use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = input("6.txt")?;

    let universe: HashMap<_, _> = input
        .lines()
        .map(|s| s.split(')').rev())
        .map(|mut p| (p.next().unwrap(), p.next().unwrap()))
        .collect();

    let you = ancestors(&universe, "YOU");
    let san = ancestors(&universe, "SAN");
    let shared = you.iter().zip(&san).take_while(|(a, b)| a == b).count();

    Ok(println!("{:?}", you.len() - shared + san.len() - shared))
}

fn ancestors<'a>(universe: &HashMap<&'a str, &'a str>, object: &'a str) -> Vec<&'a str> {
    let mut ancestors: Vec<_> = std::iter::successors(universe.get(object), |&o| universe.get(o))
        .copied()
        .collect();

    ancestors.reverse();
    ancestors
}
