use aoc::*;
use itertools::iproduct;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx, _) = Computer::load("17.txt")?.channelled();
    let mut ascii = Ascii::new(tx, rx);

    let paragraph = ascii.paragraph().await.unwrap();
    let map: Vec<_> = paragraph.lines().map(|line| line.as_bytes()).collect();

    Ok(println!("{}\n{:?}", paragraph, alignments(&map)))
}

fn alignments(map: &[&[u8]]) -> usize {
    iproduct!(1..(map[0].len() - 1), 1..(map.len() - 2))
        .filter(|&(x, y)| {
            [(x, y), (x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .iter()
                .all(|&(x, y)| map[y][x] == b'#')
        })
        .map(|(x, y)| x * y)
        .sum()
}
