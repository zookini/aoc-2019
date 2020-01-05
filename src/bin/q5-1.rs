use aoc::*;

fn main() -> Result<()> {
    Ok(println!("{:?}", Computer::load("5.txt")?.run(vec![1])?))
}
