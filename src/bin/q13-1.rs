use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let (_, rx, _) = Computer::load("13.txt")?.spawn();
    let mut histogram = [0u32; 5];

    for (_, _, v) in rx.iter().tuples() {
        histogram[v as usize] += 1;
    }

    Ok(println!("{}", histogram[2]))
}
