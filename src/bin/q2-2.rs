use aoc::*;

fn main() -> Result<()> {
    let mem: Vec<usize> = input("2.txt")?[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            if compute(mem.clone(), noun, verb) == 19_690_720 {
                println!("noun = {}, verb = {}", noun, verb);
            }
        }
    }

    Ok(())
}

fn compute(mem: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut ip = 0;
    let mut mem = mem;

    mem[1] = noun;
    mem[2] = verb;

    loop {
        match mem[ip] {
            1 => {
                let dst = mem[ip + 3];
                mem[dst] = mem[mem[ip + 1]] + mem[mem[ip + 2]];
                ip += 4;
            }
            2 => {
                let dst = mem[ip + 3];
                mem[dst] = mem[mem[ip + 1]] * mem[mem[ip + 2]];
                ip += 4;
            }
            99 => return mem[0],
            op => panic!("Unknown Op Code {}", op),
        }
    }
}
