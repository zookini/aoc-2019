use aoc::*;

fn main() -> Result<()> {
    Ok(println!("{:?}", Computer::load("9.txt")?.run(&[1])))
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

    fn run(&mut self, input: &[i64]) -> Option<i64> {
        let mut input = input.iter();

        loop {
            match self.mem[self.ip] % 100 {
                1 => {
                    *self.at(3) = *self.at(1) + *self.at(2);
                    self.ip += 4;
                }
                2 => {
                    *self.at(3) = *self.at(1) * *self.at(2);
                    self.ip += 4;
                }
                3 => {
                    *self.at(1) = *input.next().unwrap();
                    self.ip += 2;
                }
                4 => {
                    let output = *self.at(1);
                    self.ip += 2;
                    return Some(output);
                }
                5 => {
                    self.ip = if *self.at(1) != 0 {
                        *self.at(2) as usize
                    } else {
                        self.ip + 3
                    }
                }
                6 => {
                    self.ip = if *self.at(1) == 0 {
                        *self.at(2) as usize
                    } else {
                        self.ip + 3
                    }
                }
                7 => {
                    *self.at(3) = if *self.at(1) < *self.at(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                8 => {
                    *self.at(3) = if *self.at(1) == *self.at(2) { 1 } else { 0 };
                    self.ip += 4;
                }
                9 => {
                    self.base = (self.base as i64 + *self.at(1)) as usize;
                    self.ip += 2;
                }
                99 => return None,
                _ => unreachable!(),
            }
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
