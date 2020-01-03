use aoc::*;
use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("21.txt")?.spawn();
    let mut ascii = Ascii::new(tx, rx);

    println!("{}", ascii.line().await.unwrap());

    let commands = "NOT A J\n\
                    NOT B T\n\
                    OR T J\n\
                    NOT C T\n\
                    OR T J\n\
                    AND D J\n\
                    NOT H T\n\
                    NOT T T\n\
                    OR E T\n\
                    AND T J\n\
                    RUN";

    ascii.send(commands).await?;

    for _ in 0..3 {
        println!("{}", ascii.line().await.unwrap());
    }

    Ok(println!("{:?}", ascii.rx.next().await))
}
