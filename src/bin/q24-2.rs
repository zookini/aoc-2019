use aoc::*;
use itertools::iproduct;

fn main() -> Result<()> {
    let input = input("24.txt")?;
    let grid: Grid = input.lines().map(|line| line.bytes().collect()).collect();
    let mut grids = vec![grid];

    for _ in 0..200 {
        grids = step(grids);
    }

    let bugs = grids
        .iter()
        .flatten()
        .flatten()
        .filter(|&&b| b == b'#')
        .count();

    Ok(println!("{}", bugs))
}

type Grid = Vec<Vec<u8>>;

fn step(mut grids: Vec<Grid>) -> Vec<Grid> {
    grids.insert(0, vec![vec![b'.'; 5]; 5]);
    grids.push(vec![vec![b'.'; 5]; 5]);

    let mut next = grids.clone();

    for (z, y, x) in iproduct!(0..next.len(), 0..next[0].len(), 0..next[0][0].len()) {
        if (y, x) == (2, 2) {
            continue;
        }

        let bugs = neighbours(&grids, x as i8, y as i8, z);

        if grids[z][y][x] == b'#' && bugs != 1 {
            next[z][y][x] = b'.';
        } else if grids[z][y][x] == b'.' && [1, 2].contains(&bugs) {
            next[z][y][x] = b'#';
        }
    }

    next
}

fn neighbours(gs: &Vec<Grid>, x: i8, y: i8, z: usize) -> u8 {
    let width = gs[0][0].len();
    let height = gs[0].len();

    let mut count = 0;

    for &(x2, y2) in &[(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
        if z > 0 {
            if x2 < 0 && gs[z - 1][2][1] == b'#' {
                count += 1;
            } else if x2 >= width as i8 && gs[z - 1][2][3] == b'#' {
                count += 1;
            } else if y2 < 0 && gs[z - 1][1][2] == b'#' {
                count += 1;
            } else if y2 >= height as i8 && gs[z - 1][3][2] == b'#' {
                count += 1;
            }
        }
        if (x2, y2) == (2, 2) && z < gs.len() - 1 {
            if x == 1 {
                count += gs[z + 1].iter().filter(|r| r[0] == b'#').count();
            } else if x == 3 {
                count += gs[z + 1].iter().filter(|r| r[width - 1] == b'#').count();
            } else if y == 1 {
                count += gs[z + 1][0].iter().filter(|&&b| b == b'#').count();
            } else if y == 3 {
                count += gs[z + 1][height - 1].iter().filter(|&&b| b == b'#').count();
            }
        }
        if x2 >= 0
            && y2 >= 0
            && x2 < gs[0][0].len() as i8
            && y2 < gs[0].len() as i8
            && gs[z][y2 as usize][x2 as usize] == b'#'
        {
            count += 1;
        }
    }

    count as u8
}
