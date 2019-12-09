use aoc::*;

fn main() -> Result<()> {
    let fuel: f64 = input("1.txt")?
        .lines()
        .map(|s| cumulative_fuel(s.parse().unwrap()))
        .sum();

    Ok(println!("{}", fuel))
}

fn cumulative_fuel(mass: f64) -> f64 {
    let mut c = fuel(mass);
    let mut total = 0.0;

    while c > 0.0 {
        total += c;
        c = fuel(c);
    }

    total
}

fn fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
