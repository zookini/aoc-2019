use aoc::*;

fn main() -> Result<()> {
    let mut signal: Vec<i16> = input("16.txt")?
        .chars()
        .map(|ch| (ch as u8 - b'0') as i16)
        .collect();

    for _ in 0..100 {
        signal = phase(&signal);
    }

    Ok(println!("{}", extract(&signal[0..8])))
}

fn extract(bytes: &[i16]) -> usize {
    bytes.iter().fold(0, |acc, &i| acc * 10 + i as usize)
}

fn phase(signal: &[i16]) -> Vec<i16> {
    (1..=signal.len())
        .map(|step| {
            pattern(step)
                .zip(signal)
                .map(|(n, m)| n * m)
                .sum::<i16>()
                .abs()
                % 10
        })
        .collect()
}

fn pattern(step: usize) -> impl Iterator<Item = i16> {
    [0, 1, 0, -1]
        .iter()
        .cycle()
        .flat_map(move |i| std::iter::repeat(*i).take(step))
        .skip(1)
}
