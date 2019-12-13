use itertools::{izip, Itertools};

fn main() {
    let x = orbit(&[17, -2, 7, 1]);
    let y = orbit(&[5, -8, -6, -10]);
    let z = orbit(&[1, 8, 14, 4]);

    println!("{}", lcm(x, lcm(y, z)));
}

fn orbit(start: &[i64; 4]) -> i64 {
    let mut positions = *start;
    let mut velocities = [0; 4];

    for steps in 1.. {
        step(&mut positions, &mut velocities);

        if positions == *start {
            return steps + 1;
        }
    }

    unreachable!()
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
