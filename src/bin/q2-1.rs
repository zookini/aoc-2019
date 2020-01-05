use aoc::*;

fn main() -> Result<()> {
    let mut computer = Computer::load("2.txt")?;

    computer.mem[1] = 12;
    computer.mem[2] = 2;
    computer.run(vec![])?;

    Ok(println!("{}", computer.mem[0]))
}
