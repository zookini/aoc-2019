use aoc::*;
use itertools::iproduct;

fn main() -> Result<()> {
    let vm = Computer::load("19.txt")?;

    let points: i64 = iproduct!(0..50, 0..50)
        .filter_map(|(x, y)| vm.clone().run(vec![x, y]).ok().map(|v| v[0]))
        .sum();

    Ok(println!("{:?}", points))
}
