use aoc::*;

fn main() -> Result<()> {
    let mut signal: Vec<_> = input("16.txt")?.bytes().map(|b| b - b'0').collect();

    for _ in 0..100 {
        signal = phase(&signal);
    }

    Ok(println!("{}", extract(&signal[0..8])))
}

fn extract(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |acc, &i| acc * 10 + i as usize)
}

fn phase(signal: &[u8]) -> Vec<u8> {
    (1..=signal.len())
        .map(|step| {
            pattern(step)
                .zip(signal)
                .map(|(n, &m)| n as i16 * m as i16)
                .sum::<i16>()
                .abs()
                % 10
        } as u8)
        .collect()
}

fn pattern(step: usize) -> impl Iterator<Item = i8> {
    [0, 1, 0, -1]
        .iter()
        .cycle()
        .flat_map(move |i| std::iter::repeat(*i).take(step))
        .skip(1)
}
