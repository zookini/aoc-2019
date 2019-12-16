use aoc::*;
use std::iter;

fn main() -> Result<()> {
    let input: Vec<i64> = input("16.txt")?
        .bytes()
        .map(|b| (b - b'0') as i64)
        .collect();

    let offset = extract(&input[0..7]);

    let mut signal: Vec<i64> = iter::repeat(input)
        .take(10000)
        .flat_map(|a| a.into_iter())
        .skip(offset)
        .collect();

    for _ in 0..100 {
        phase(&mut signal);
    }

    Ok(println!("{}", extract(&signal[0..8])))
}

fn extract(bytes: &[i64]) -> usize {
    bytes.iter().fold(0, |acc, &i| acc * 10 + i as usize)
}

fn phase(signal: &mut [i64]) {
    let mut sum: i64 = signal.iter().sum();

    for s in signal {
        let tmp = sum;
        sum -= *s;
        *s = tmp % 10;
    }
}
