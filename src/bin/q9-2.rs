use aoc::*;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(println!("{:?}", Computer::load("9.txt")?.run(vec![2])))
}
