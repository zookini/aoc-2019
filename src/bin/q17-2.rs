use aoc::*;
use itertools::izip;
use std::iter;

fn main() -> Result<()> {
    let mut computer = Computer::load("17.txt")?;
    computer.mem[0] = 2;

    let map: Vec<_> = computer.lines();
    println!("{}\n", map.join("\n"));

    let map: Vec<_> = map.iter().map(|line| line.as_bytes()).collect();
    let mut robot = Robot::load(&map);
    let moves: Vec<_> = iter::from_fn(|| robot.step(&map)).collect();

    println!("{:?}", moves);
    println!("{}", computer.line(&[])); // Main

    // Manually computed routines from moves

    for input in &[
        "A,A,B,C,B,C,B,C,B,A",
        "R,10,L,12,R,6",
        "R,6,R,10,R,12,R,6",
        "R,10,L,12,L,12",
        "n",
    ] {
        println!("{}", computer.line(&ascii(input)));
    }

    let map: Vec<_> = computer.lines();

    println!("{}\n", map.join("\n"));
    println!("{:?}", computer.run(&[]));

    Ok(())
}

fn ascii(s: &str) -> Vec<i64> {
    s.bytes().map(|b| b as i64).chain(iter::once(10)).collect()
}

#[derive(Debug)]
struct Robot {
    position: (i8, i8),
    facing: (i8, i8),
}

impl Robot {
    fn load(map: &[&[u8]]) -> Self {
        for (y, &row) in map.iter().enumerate() {
            for (x, ch) in row.iter().enumerate() {
                let direction = match ch {
                    b'>' => (1, 0),
                    b'v' => (0, 1),
                    b'<' => (-1, 0),
                    b'^' => (0, -1),
                    _ => continue,
                };

                return Self {
                    position: (x as i8, y as i8),
                    facing: direction,
                };
            }
        }

        unreachable!()
    }

    fn step(&mut self, map: &[&[u8]]) -> Option<(char, u8)> {
        let moves = match self.facing {
            (1, 0) => [(0, -1), (0, 1)],
            (0, 1) => [(1, 0), (-1, 0)],
            (-1, 0) => [(0, 1), (0, -1)],
            (0, -1) => [(-1, 0), (1, 0)],
            _ => unreachable!(),
        };

        for (&direction, &facing) in izip!(&['L', 'R'], &moves) {
            let mut distance = 0;
            let mut position = (self.position.0 + facing.0, self.position.1 + facing.1);

            while position.0 >= 0
                && position.1 >= 0
                && (position.0 as usize) < map[0].len()
                && (position.1 as usize) < map.len()
                && map[position.1 as usize][position.0 as usize] == b'#'
            {
                position = (position.0 + facing.0, position.1 + facing.1);
                distance += 1;
            }

            if distance > 0 {
                self.position = (position.0 - facing.0, position.1 - facing.1);
                self.facing = facing;
                return Some((direction, distance));
            }
        }

        None
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
