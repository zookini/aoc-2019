use aoc::*;

fn main() -> Result<()> {
    let input = input("8.txt")?;

    let layer = input
        .as_bytes()
        .chunks(25 * 6)
        .min_by_key(|layer| count(layer, b'0'))
        .unwrap();

    Ok(println!("{}", count(layer, b'1') * count(layer, b'2')))
}

fn count(layer: &[u8], b: u8) -> usize {
    layer.iter().filter(|&&a| a == b).count()
}
