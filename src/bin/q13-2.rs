use aoc::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut computer = Computer::load("13.txt")?;

    computer.mem[0] = 2;

    let (mut tx, rx, _) = computer.channelled();
    let mut updates = rx.chunks(3).map(|v| (v[0], v[1], v[2]));

    let mut ball;
    let mut paddle = (0, 0);
    let mut score = 0;

    while let Some((x, y, object)) = updates.next().await {
        match (x, object) {
            (-1, _) => score = object,
            (_, 3) => paddle = (x, y),
            (_, 4) => {
                ball = (x, y);
                tx.send(ball.0.cmp(&paddle.0) as i64).await?;
            }
            (_, _) => (),
        }
    }

    Ok(println!("{}", score))
}
