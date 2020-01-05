use aoc::*;
use std::sync::mpsc::{Receiver, Sender};

fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("15.txt")?.spawn();
    let mut grid = vec![vec![255; 50]; 50];

    let mut droid = Droid {
        tx,
        rx,
        position: Point::new(25, 25),
    };

    let distance = dfs(&mut grid, &mut droid, 1);
    draw(&mut grid);

    Ok(println!("{:?} steps to oxygen", distance))
}

struct Droid {
    tx: Sender<i64>,
    rx: Receiver<i64>,
    position: Point,
}

impl Droid {
    fn go(&mut self, direction: i64) -> Option<i64> {
        self.tx.send(direction).unwrap();
        self.rx.recv().ok()
    }
}

fn dfs(grid: &mut Vec<Vec<u8>>, droid: &mut Droid, distance: u16) -> Option<u16> {
    for direction in 1..=4 {
        let destination = droid.position.direction(direction);

        if grid[destination.y][destination.x] != 255 {
            continue;
        }

        if let Some(response) = droid.go(direction) {
            grid[destination.y][destination.x] = response as u8;

            match response {
                0 => continue,
                2 => return Some(distance),
                _ => (),
            }

            droid.position = destination;

            if let Some(distance) = dfs(grid, droid, distance + 1) {
                return Some(distance);
            }

            let backtrack = match direction {
                1 => 2,
                2 => 1,
                3 => 4,
                4 => 3,
                _ => unreachable!(),
            };

            droid.go(backtrack);
            droid.position = droid.position.direction(backtrack);
        }
    }

    None
}

fn draw(grid: &Vec<Vec<u8>>) {
    for row in grid {
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

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Self { x, y }
    }

    fn direction(&self, direction: i64) -> Point {
        match direction {
            1 => Self::new(self.x, self.y - 1),
            2 => Self::new(self.x, self.y + 1),
            3 => Self::new(self.x - 1, self.y),
            4 => Self::new(self.x + 1, self.y),
            _ => unreachable!(),
        }
    }
}
