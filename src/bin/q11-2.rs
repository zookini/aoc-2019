use aoc::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut computer = Computer::load("11.txt")?;
    let mut position = (0, 0);
    let mut facing = 0;
    let mut painted = HashMap::new();

    painted.insert(position, 1);

    while let Some(colour) = computer.run(&[*painted.get(&position).unwrap_or(&0)]) {
        painted.insert(position, colour);

        match computer.run(&[]) {
            Some(direction) => {
                facing = turn(facing, direction);
                position = step(position, facing);
            }
            _ => break,
        }
    }

    Ok(draw(&painted))
}

type Point = (isize, isize);

fn turn(facing: i64, input: i64) -> i64 {
    (facing + if input == 0 { 270 } else { 90 }) % 360
}

fn step((x, y): Point, direction: i64) -> Point {
    match direction {
        0 => (x, y - 1),
        90 => (x + 1, y),
        180 => (x, y + 1),
        270 => (x - 1, y),
        _ => unreachable!(),
    }
}

fn draw(painted: &HashMap<Point, i64>) {
    let whites: Vec<_> = painted
        .iter()
        .filter(|(_, c)| **c == 1)
        .map(|(p, _)| *p)
        .collect();

    let min = whites
        .iter()
        .fold(whites[0], |a, b| (a.0.min(b.0), a.1.min(b.1)));

    let whites: Vec<_> = whites.iter().map(|(x, y)| (x - min.0, y - min.1)).collect();

    let max = whites
        .iter()
        .fold(whites[0], |a, b| (a.0.max(b.0), a.1.max(b.1)));

    let mut canvas = vec![vec![b' '; max.0 as usize + 1]; max.1 as usize + 1];

    for (x, y) in whites {
        canvas[y as usize][x as usize] = b'#';
    }

    for row in canvas {
        println!("{}", std::str::from_utf8(&row).unwrap());
    }
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
