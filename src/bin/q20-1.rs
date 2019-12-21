use aoc::*;
use derive_more::Add;
use itertools::{iproduct, Itertools};
use std::collections::{HashMap, HashSet};
use std::iter::once;

fn main() -> Result<()> {
    let input = input("20.txt")?;
    Ok(println!("Travelled {} steps", Grid::new(&input).bfs()))
}

struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn new(s: &str) -> Self {
        Self {
            grid: s.lines().map(|line| line.as_bytes().to_vec()).collect(),
        }
    }

    fn bfs(&mut self) -> u32 {
        let portals = iproduct!(0..self.grid[0].len() - 1, 0..self.grid.len() - 1)
            .filter_map(|(x, y)| self.portal(x, y))
            .into_group_map();

        let teleports: HashMap<Point, Point> = portals
            .values()
            .filter(|v| v.len() == 2)
            .flat_map(|v| once((v[0], v[1])).chain(once((v[1], v[0]))))
            .collect();

        let mut border = vec![portals["AA"][0]];
        let mut visited = HashSet::new();
        let finish = portals["ZZ"][0];

        for travelled in 0.. {
            let mut next = vec![];

            for position in border {
                if position == finish {
                    return travelled;
                }

                visited.insert(position);

                for direction in Point::DIRECTIONS {
                    let go = position + *direction;
                    let entity = self.at(go);

                    if visited.contains(&go) {
                        continue;
                    } else if *entity == b'.' {
                        next.push(go);
                    } else if entity.is_ascii_alphabetic() {
                        teleports.get(&position).map(|p| next.push(*p));
                    }
                }
            }

            border = next;
        }

        unreachable!()
    }

    fn portal(&self, x: usize, y: usize) -> Option<(String, Point)> {
        if !self.grid[y][x].is_ascii_alphabetic() {
            None
        } else if self.grid[y + 1][x].is_ascii_alphabetic() {
            Some((
                self.grid[y + 1][x],
                if y > 1 && self.grid[y - 1][x] == b'.' {
                    (x, y - 1)
                } else {
                    (x, y + 2)
                },
            ))
        } else if self.grid[y][x + 1].is_ascii_alphabetic() {
            Some((
                self.grid[y][x + 1],
                if x > 1 && self.grid[y][x - 1] == b'.' {
                    (x - 1, y)
                } else {
                    (x + 2, y)
                },
            ))
        } else {
            None
        }
        .map(|(b, (x2, y2))| {
            (
                String::from_utf8_lossy(&[self.grid[y][x], b]).into(),
                Point::new(x2 as i16, y2 as i16),
            )
        })
    }

    fn at(&mut self, position: Point) -> &mut u8 {
        &mut self.grid[position.y as usize][position.x as usize]
    }
}

#[derive(Add, Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    const DIRECTIONS: &'static [Point] = &[
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    ];

    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}
