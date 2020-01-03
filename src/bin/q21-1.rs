use aoc::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("21.txt")?.spawn();
    let mut ascii = Ascii::new(tx, rx);

    println!("{}", ascii.line().await.unwrap());

    for command in &["NOT A J", "NOT C T", "AND D T", "OR T J", "WALK"] {
        println!("Send {}", command);
        ascii.send(command).await?;
    }

    for _ in 0..3 {
        println!("{}", ascii.line().await.unwrap());
    }

    Ok(println!("{:?}", ascii.rx.next().await))
}
