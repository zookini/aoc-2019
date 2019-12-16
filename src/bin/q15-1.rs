use aoc::*;

fn main() -> Result<()> {
    let mut world = World {
        computer: Computer::load("15.txt")?,
        map: vec![vec![255; 50]; 50],
        position: Point::new(25, 25),
    };

    world.dfs(1);
    world.draw();

    Ok(())
}

struct World {
    computer: Computer,
    map: Vec<Vec<u8>>,
    position: Point,
}

impl World {
    fn dfs(&mut self, distance: u16) {
        for direction in 1..=4 {
            let destination = self.position.direction(direction);

            if self.map[destination.y][destination.x] != 255 {
                continue;
            }

            if let Some(response) = self.computer.run(&[direction]) {
                self.map[destination.y][destination.x] = response as u8;

                match response {
                    0 => continue,
                    2 => println!("Distance to oxygen: {}", distance),
                    _ => (),
                }

                self.position = destination;
                self.dfs(distance + 1);

                let backtrack = match direction {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    4 => 3,
                    _ => unreachable!(),
                };

                self.computer.run(&[backtrack]);
                self.position = self.position.direction(backtrack);
            }
        }
    }

    fn draw(&self) {
        for row in &self.map {
            let line: String = row
                .iter()
                .map(|b| match b {
                    0 => '#',
                    1 => '.',
                    2 => '*',
                    _ => ' ',
                })
                .collect();

            println!("{}", line);
        }
    }
}
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn direction(&self, direction: i64) -> Point {
        match direction {
            1 => Point::new(self.x, self.y - 1),
            2 => Point::new(self.x, self.y + 1),
            3 => Point::new(self.x - 1, self.y),
            4 => Point::new(self.x + 1, self.y),
            _ => unreachable!(),
        }
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
