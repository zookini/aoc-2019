use aoc::*;

fn main() -> Result<()> {
    let fuel: f64 = input("1.txt")?
        .iter()
        .map(|s| fuel(s.parse().unwrap()))
        .sum();

    Ok(println!("{}", fuel))
}

fn fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
