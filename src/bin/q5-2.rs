use aoc::*;
use std::io;

fn main() -> Result<()> {
    Ok(Computer::load("5.txt")?.run()?)
}

struct Computer {
    mem: Vec<i32>,
    ip: usize,
}

impl Computer {
    fn load(filename: &str) -> Result<Self> {
        Ok(Computer {
            mem: input(filename)?
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            ip: 0,
        })
    }

    fn read(&self, parameter: usize) -> i32 {
        let immediate = self.mem[self.ip] / 10i32.pow(parameter as u32 + 1) % 10 == 1;
        self.mem[if immediate {
            self.ip + parameter
        } else {
            self.mem[self.ip + parameter] as usize
        }]
    }

    fn run(&mut self) -> Result<()> {
        loop {
            match self.mem[self.ip] % 100 {
                1 => {
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = self.read(1) + self.read(2);
                    self.ip += 4;
                }
                2 => {
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = self.read(1) * self.read(2);
                    self.ip += 4;
                }
                3 => {
                    eprint!("Op - Input: ");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let dst = self.mem[self.ip + 1] as usize;
                    self.mem[dst] = input.trim().parse()?;
                    self.ip += 2;
                }
                4 => {
                    println!("Op - Output: {}", self.read(1));
                    self.ip += 2;
                }
                5 => {
                    self.ip = if self.read(1) != 0 {
                        self.read(2) as usize
                    } else {
                        self.ip + 3
                    }
                }
                6 => {
                    self.ip = if self.read(1) == 0 {
                        self.read(2) as usize
                    } else {
                        self.ip + 3
                    }
                }
                7 => {
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if self.read(1) < self.read(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    let dst = self.mem[self.ip + 3] as usize;
                    self.mem[dst] = if self.read(1) == self.read(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                99 => return Ok(()),
                _ => unreachable!(),
            }
        }
    }
}
