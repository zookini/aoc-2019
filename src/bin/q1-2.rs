use aoc::*;

fn main() -> Result<()> {
    let fuel: f64 = input("1.txt")?
        .lines()
        .map(|s| {
            let mut c = fuel(s.parse().unwrap());
            let mut total = c;

            loop {
                c = fuel(c);
                if c > 0.0 {
                    total += c;
                } else {
                    return total;
                }
            }
        })
        .sum();

    Ok(println!("{}", fuel))
}

fn fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
