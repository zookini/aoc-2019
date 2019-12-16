use aoc::*;
use std::iter;

fn main() -> Result<()> {
    let input: Vec<_> = input("16.txt")?.bytes().map(|b| b - b'0').collect();
    let offset = extract(&input[0..7]);

    let mut signal: Vec<_> = iter::repeat(input)
        .take(10000)
        .flatten()
        .skip(offset)
        .collect();

    for _ in 0..100 {
        phase(&mut signal);
    }

    Ok(println!("{}", extract(&signal[0..8])))
}

fn extract(digits: &[u8]) -> usize {
    digits.iter().fold(0, |acc, &i| acc * 10 + i as usize)
}

fn phase(signal: &mut [u8]) {
    let mut sum: usize = 0;

    for s in signal.iter_mut().rev() {
        sum += *s as usize;
        *s = (sum % 10) as u8;
    }
}
