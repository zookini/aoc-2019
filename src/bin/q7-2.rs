use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let image = Computer::load("7.txt")?;
    let signal = (5..10).permutations(5).map(|p| amplify(&image, &p)).max();

    Ok(println!("{:?}", signal))
}

fn amplify(image: &Computer, phases: &[i32]) -> i32 {
    let mut amplifiers: Vec<_> = phases.iter().map(|_| image.clone()).collect();

    let init = phases
        .iter()
        .zip(&mut amplifiers)
        .fold(0, |input, (p, a)| a.run(&[*p, input]).unwrap());

    (0..amplifiers.len())
        .cycle()
        .try_fold(init, |input, i| amplifiers[i].run(&[input]));

    amplifiers.last().unwrap().output
}

#[derive(Clone)]
struct Computer {
    mem: Vec<i32>,
    ip: usize,
    output: i32,
}

impl Computer {
    fn load(filename: &str) -> Result<Self> {
        Ok(Computer {
            mem: input(filename)?
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
            ip: 0,
            output: 0,
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

    fn run(&mut self, input: &[i32]) -> Option<i32> {
        let mut input = input.iter();
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
                    let dst = self.mem[self.ip + 1] as usize;
                    self.mem[dst] = *input.next().unwrap();
                    self.ip += 2;
                }
                4 => {
                    self.output = self.read(1);
                    self.ip += 2;
                    return Some(self.output);
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
                99 => return None,
                _ => unreachable!(),
            }
        }
    }
}
