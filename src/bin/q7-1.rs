use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let image = Computer::load("7.txt")?;
    let signal = (0..5).permutations(5).map(|p| amplify(&image, &p)).max();

    Ok(println!("{:?}", signal))
}

fn amplify(image: &Computer, phases: &[i64]) -> i64 {
    phases.iter().fold(0, |input, phase| {
        image.clone().run(vec![*phase, input]).unwrap()[0]
    })
}
