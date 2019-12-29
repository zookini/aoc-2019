use aoc::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(println!("{:?}", Computer::load("5.txt")?.run(vec![1])?))
}
