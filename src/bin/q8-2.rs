use aoc::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> Result<()> {
    let input = input("8.txt")?;
    let image = collapse(input.as_bytes().chunks(WIDTH * HEIGHT));

    Ok(draw(&image))
}

fn collapse<'a>(layers: impl Iterator<Item = &'a [u8]>) -> Vec<u8> {
    layers.fold(vec![b'2'; WIDTH * HEIGHT], |mut image, layer| {
        for (i, l) in image.iter_mut().zip(layer) {
            if *i == b'2' {
                *i = *l;
            }
        }

        image
    })
}

fn draw(image: &[u8]) {
    for row in image.chunks(WIDTH) {
        println!("{}", std::str::from_utf8(row).unwrap().replace('0', " "));
    }
}
