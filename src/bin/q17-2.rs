use aoc::*;
use itertools::izip;
use std::iter;

fn main() -> Result<()> {
    let mut computer = Computer::load("17.txt")?;
    computer.mem[0] = 2;

    let (tx, rx, _) = computer.spawn();
    let mut ascii = Ascii::new(tx, rx);

    let map = ascii.paragraph().unwrap();
    println!("{}\n", map);

    let map: Vec<_> = map.lines().map(|line| line.as_bytes()).collect();
    let mut robot = Robot::load(&map);
    let moves: Vec<_> = iter::from_fn(|| robot.step(&map)).collect();

    println!("{:?}\n", moves);
    println!("{}", ascii.line().unwrap()); // Main

    // Manually computed routines from moves

    for input in &[
        "A,A,B,C,B,C,B,C,B,A",
        "R,10,L,12,R,6",
        "R,6,R,10,R,12,R,6",
        "R,10,L,12,L,12",
        "n",
    ] {
        ascii.send(input)?;
        println!("{}", ascii.line().unwrap());
    }

    let map = ascii.paragraph().unwrap();

    println!("{}\n", map);
    println!("{:?}", ascii.rx.iter().collect::<Vec<_>>());
    Ok(())
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
