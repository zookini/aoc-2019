use itertools::{izip, Itertools};

fn main() {
    let mut positions = [[17, 5, 1], [-2, -8, 8], [7, -6, 14], [1, -10, 4]];
    let mut velocities = [[0; 3]; 4];

    for _ in 0..1000 {
        step(&mut positions, &mut velocities);
    }

    let energy: i32 = izip!(&positions, &velocities)
        .map(|(p, v)| energy(p) * energy(v))
        .sum();

    println!("{:#?}", energy);
}

type Point = [i32; 3];

fn step(positions: &mut [Point], velocities: &mut [Point]) {
    for (a, b) in (0..positions.len()).tuple_combinations() {
        gravity(&positions[a], &positions[b], &mut velocities[a]);
        gravity(&positions[b], &positions[a], &mut velocities[b]);
    }

    for (position, velocity) in izip!(positions, velocities) {
        for (p, v) in izip!(position, velocity) {
            *p += *v;
        }
    }
}

fn gravity(point1: &Point, point2: &Point, velocity: &mut Point) {
    for (p1, p2, v) in izip!(point1, point2, velocity) {
        *v += p2.cmp(&p1) as i32;
    }
}

fn energy(point: &Point) -> i32 {
    point.iter().map(|i| i.abs()).sum()
}
