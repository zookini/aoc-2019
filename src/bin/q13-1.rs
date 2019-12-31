use aoc::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (_, rx, _) = Computer::load("13.txt")?.channelled();
    let mut updates = rx.chunks(3);
    let mut histogram = [0u32; 5];

    while let Some(v) = updates.next().await {
        histogram[v[2] as usize] += 1;
    }

    Ok(println!("{}", histogram[2]))
}
