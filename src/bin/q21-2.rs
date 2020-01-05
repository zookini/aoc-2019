use aoc::*;

fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("21.txt")?.spawn();
    let mut ascii = Ascii::new(tx, rx);

    println!("{}", ascii.line().unwrap());

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

    ascii.send(commands)?;

    for _ in 0..3 {
        println!("{}", ascii.line().unwrap());
    }

    Ok(println!("{}", ascii.rx.recv().unwrap()))
}
