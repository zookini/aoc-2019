use aoc::*;
use std::iter;

fn main() -> Result<()> {
    let mut computer = Computer::load("21.txt")?;
    let commands = &["NOT A J", "NOT C T", "AND D T", "OR T J", "WALK"].join("\n");

    println!("{}", computer.line(&[]));
    println!("{}", computer.line(&ascii(commands)));
    println!("{}", computer.lines().join("\n"));
    println!("{:?}", computer.run(&[]));
    Ok(())
}

fn ascii(s: &str) -> Vec<i64> {
    s.bytes().map(|b| b as i64).chain(iter::once(10)).collect()
}

#[derive(Clone)]
struct Computer {
    base: usize,
    mem: Vec<i64>,
    ip: usize,
}

impl Computer {
    fn load(filename: &str) -> Result<Self> {
        let mut mem: Vec<i64> = input(filename)?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        mem.resize(10 * 1024, 0);

        Ok(Computer {
            base: 0,
            mem,
            ip: 0,
        })
    }

    fn lines(&mut self) -> Vec<String> {
        iter::from_fn(|| Some(self.line(&[])).filter(|s| !s.is_empty())).collect()
    }

    fn line(&mut self, input: &[i64]) -> String {
        iter::from_fn(|| self.run(input))
            .map(|i| i as u8 as char)
            .take_while(|&ch| ch != '\n')
            .collect()
    }

    const OP_SIZE: &'static [usize] = &[0, 4, 4, 2, 2, 3, 3, 4, 4, 2];

    fn run(&mut self, input: &[i64]) -> Option<i64> {
        let mut input = input.iter();

        loop {
            let op = (self.mem[self.ip] % 100) as usize;

            match op {
                1 => *self.at(3) = *self.at(1) + *self.at(2),
                2 => *self.at(3) = *self.at(1) * *self.at(2),
                3 => *self.at(1) = *input.next().unwrap(),
                4 => {
                    let output = *self.at(1);
                    self.ip += Self::OP_SIZE[op];
                    return Some(output);
                }
                5 => {
                    if *self.at(1) != 0 {
                        self.ip = *self.at(2) as usize;
                        continue;
                    }
                }
                6 => {
                    if *self.at(1) == 0 {
                        self.ip = *self.at(2) as usize;
                        continue;
                    }
                }
                7 => *self.at(3) = if *self.at(1) < *self.at(2) { 1 } else { 0 },
                8 => *self.at(3) = if *self.at(1) == *self.at(2) { 1 } else { 0 },
                9 => self.base = (self.base as i64 + *self.at(1)) as usize,
                99 => return None,
                _ => unreachable!(),
            }

            self.ip += Self::OP_SIZE[op];
        }
    }

    fn at(&mut self, parameter: usize) -> &mut i64 {
        let mode = self.mem[self.ip] / 10i64.pow(parameter as u32 + 1) % 10;
        let parameter = self.ip + parameter;

        let address = match mode {
            0 => self.mem[parameter] as usize,
            1 => parameter,
            2 => (self.base as i64 + self.mem[parameter]) as usize,
            _ => unreachable!(),
        };

        &mut self.mem[address]
    }
}
