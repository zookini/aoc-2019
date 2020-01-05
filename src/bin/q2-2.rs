use aoc::*;

fn main() -> Result<()> {
    let mem = Computer::parse("2.txt")?;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = Computer::new(mem.clone());

            computer.mem[1] = noun;
            computer.mem[2] = verb;
            computer.run(vec![])?;

            if computer.mem[0] == 19_690_720 {
                println!("noun = {}, verb = {}", noun, verb);
            }
        }
    }

    Ok(())
}
