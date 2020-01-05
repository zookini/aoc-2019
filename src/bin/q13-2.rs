use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut computer = Computer::load("13.txt")?;

    computer.mem[0] = 2;

    let (tx, rx, _) = computer.spawn();

    let mut ball;
    let mut paddle = (0, 0);
    let mut score = 0;

    for (x, y, object) in rx.iter().tuples() {
        match (x, object) {
            (-1, _) => score = object,
            (_, 3) => paddle = (x, y),
            (_, 4) => {
                ball = (x, y);
                tx.send(ball.0.cmp(&paddle.0) as i64)?;
            }
            (_, _) => (),
        }
    }

    Ok(println!("{}", score))
}
