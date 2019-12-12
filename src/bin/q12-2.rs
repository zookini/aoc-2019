use itertools::{izip, Itertools};

fn main() {
    let input = [[17, -2, 7, 1], [5, -8, -6, -10], [1, 8, 14, 4]];
    let mut positions = input;
    let mut velocities = [[0; 4]; 3];
    let mut orbits = [0; 3];

    for dimension in 0..input.len() {
        for steps in 1.. {
            step(&mut positions[dimension], &mut velocities[dimension]);

            if positions[dimension] == input[dimension] {
                orbits[dimension] = steps + 1;
                break;
            }
        }
    }

    println!("{}", lcm(lcm(orbits[0], orbits[1]), orbits[2]));
}

fn step(positions: &mut [i64], velocities: &mut [i64]) {
    for (a, b) in (0..positions.len()).tuple_combinations() {
        velocities[a] += (positions[b] - positions[a]).signum();
        velocities[b] += (positions[a] - positions[b]).signum();
    }

    for (position, velocity) in izip!(positions, velocities) {
        *position += *velocity;
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
