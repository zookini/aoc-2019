use aoc::*;
use derive_more::Add;
use std::collections::{hash_map::Entry, BTreeSet, HashMap};

fn main() -> Result<()> {
    let input = input("18.txt")?;
    let mut grid = Grid::load(&input);
    let mut total = 0;

    grid.segregate();

    for grid in grid.quarter() {
        let mut state = State::new(&grid);

        grid.draw();
        grid.bfs(&mut state, grid.start().unwrap(), 0);

        total += state.best;
    }

    Ok(println!("total {}", total))
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn load(input: &str) -> Self {
        Self {
            grid: input.lines().map(|line| line.as_bytes().to_vec()).collect(),
        }
    }

    fn start(&self) -> Option<Point> {
        self.find(|b| b == b'@')
    }

    fn segregate(&mut self) {
        let start = self.start().unwrap();

        *self.at_mut(start) = b'#';

        for point in Point::DIRECTIONS {
            *self.at_mut(start + *point) = b'#';
        }

        for &(x, y) in &[(-1, -1), (1, 1), (1, -1), (-1, 1)] {
            *self.at_mut(start + Point::new(x, y)) = b'@';
        }
    }

    fn quarter(&mut self) -> Vec<Grid> {
        let step = (self.grid[0].len() / 2, self.grid.len() / 2);

        [(0, 0), (step.0, 0), (0, step.1), (step.0, step.1)]
            .iter()
            .map(|&(x, y)| Self {
                grid: self
                    .grid
                    .iter()
                    .skip(y)
                    .map(|row| row.iter().skip(x).take(step.0 + 1).copied().collect())
                    .take(step.1 + 1)
                    .collect::<Vec<_>>(),
            })
            .collect()
    }

    fn bfs(&self, state: &mut State, start: Point, travelled: u32) {
        let mut border = vec![start];

        for i in travelled.. {
            if i + 1 >= state.best {
                return;
            }
            if state.needed.is_empty() {
                println!("best so far = {}", i);
                state.best = i;
                return;
            }

            let mut next = vec![];

            for position in border {
                if !state.memoise(position, i) {
                    continue;
                }

                for direction in Point::DIRECTIONS {
                    let movement = position + *direction;
                    let entity = self.at(movement);

                    if state.needed.remove(&entity) {
                        self.bfs(state, movement, i + 1);
                        state.needed.insert(entity);
                    } else if entity != b'#' && !state.needed.contains(&entity.to_ascii_lowercase())
                    {
                        next.push(movement);
                    }
                }
            }

            border = next;
        }
    }

    fn draw(&self) {
        for row in &self.grid {
            let line: Vec<_> = row
                .iter()
                .map(|&b| if b >= 128 { b'?' } else { b })
                .collect();

            println!("{}", std::str::from_utf8(&line).unwrap());
        }
    }

    fn find(&self, p: impl Fn(u8) -> bool) -> Option<Point> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                if p(b) {
                    return Some(Point::new(x as i8, y as i8));
                }
            }
        }

        None
    }

    fn keys(&self) -> impl Iterator<Item = u8> + '_ {
        self.grid
            .iter()
            .flatten()
            .filter(|b| b.is_ascii_lowercase())
            .copied()
    }

    fn at(&self, position: Point) -> u8 {
        self.grid[position.y as usize][position.x as usize]
    }

    fn at_mut(&mut self, position: Point) -> &mut u8 {
        &mut self.grid[position.y as usize][position.x as usize]
    }
}

struct State {
    best: u32,
    memo: HashMap<(Point, String), u32>,
    needed: BTreeSet<u8>,
}

impl State {
    fn new(grid: &Grid) -> Self {
        Self {
            best: std::u32::MAX,
            memo: HashMap::new(),
            needed: grid.keys().collect(),
        }
    }

    fn memoise(&mut self, position: Point, distance: u32) -> bool {
        let key = (
            position,
            self.needed.iter().map(|&b| b as char).collect::<String>(),
        );

        match self.memo.entry(key) {
            Entry::Vacant(e) => {
                e.insert(distance);
                true
            }
            Entry::Occupied(mut e) => {
                let d = e.get_mut();

                if *d > distance {
                    *d = distance;
                    true
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Add, Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i8,
    y: i8,
}

impl Point {
    const DIRECTIONS: &'static [Point] = &[
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    ];

    fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}
