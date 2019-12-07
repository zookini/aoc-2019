use aoc::*;

fn main() -> Result<()> {
    let mut ip = 0;
    let mut mem: Vec<usize> = input("2.txt")?[0]
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    mem[1] = 12;
    mem[2] = 2;

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
            99 => break,
            op => panic!("Unknown Op Code {}", op),
        }
    }

    Ok(eprintln!("ip: {} mem: {:?}", ip, mem))
}
