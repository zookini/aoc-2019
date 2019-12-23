use aoc::*;
use std::collections::VecDeque;

fn main() -> Result<()> {
    let image = Computer::load("23.txt")?;

    let mut nics: Vec<_> = (0..50)
        .map(|i| {
            let mut nic = image.clone();
            nic.input.push_back(i);
            nic
        })
        .collect();

    loop {
        if let Some(y) = step(&mut nics) {
            println!("{}", y);
            break;
        }
    }

    Ok(())
}

fn step(nics: &mut [Computer]) -> Option<i64> {
    for i in 0..nics.len() {
        if let State::Output(destination) = nics[i].step() {
            if let (Some(x), Some(y)) = (nics[i].run(), nics[i].run()) {
                let destination = destination as usize;

                if destination < nics.len() {
                    nics[destination].send((x, y));
                } else if destination == 255 {
                    return Some(y);
                }
            }
        }
    }

    None
}

type Packet = (i64, i64);

#[derive(Clone)]
struct Computer {
    base: usize,
    mem: Vec<i64>,
    ip: usize,
    input: VecDeque<i64>,
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
            input: VecDeque::new(),
        })
    }

    fn send(&mut self, (x, y): Packet) {
        self.input.push_back(x);
        self.input.push_back(y);
    }

    fn run(&mut self) -> Option<i64> {
        loop {
            match self.step() {
                State::Output(output) => return Some(output),
                State::Complete => return None,
                _ => (),
            }
        }
    }

    const OP_SIZE: &'static [usize] = &[0, 4, 4, 2, 2, 3, 3, 4, 4, 2];

    fn step(&mut self) -> State {
        let op = (self.mem[self.ip] % 100) as usize;

        match op {
            1 => *self.at(3) = *self.at(1) + *self.at(2),
            2 => *self.at(3) = *self.at(1) * *self.at(2),
            3 => {
                *self.at(1) = self.input.pop_front().unwrap_or(-1);
                self.ip += Self::OP_SIZE[op];
                return State::Input;
            }
            4 => {
                let output = *self.at(1);
                self.ip += Self::OP_SIZE[op];
                return State::Output(output);
            }
            5 => {
                if *self.at(1) != 0 {
                    self.ip = *self.at(2) as usize;
                    return State::Continue;
                }
            }
            6 => {
                if *self.at(1) == 0 {
                    self.ip = *self.at(2) as usize;
                    return State::Continue;
                }
            }
            7 => *self.at(3) = if *self.at(1) < *self.at(2) { 1 } else { 0 },
            8 => *self.at(3) = if *self.at(1) == *self.at(2) { 1 } else { 0 },
            9 => self.base = (self.base as i64 + *self.at(1)) as usize,
            99 => return State::Complete,
            _ => unreachable!(),
        }

        self.ip += Self::OP_SIZE[op];
        State::Continue
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

#[derive(Eq, PartialEq)]
enum State {
    Continue,
    Input,
    Output(i64),
    Complete,
}
