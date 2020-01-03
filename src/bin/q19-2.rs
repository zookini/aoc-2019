use aoc::*;

const SIZE: i64 = 100;

fn main() -> Result<()> {
    let vm = Computer::load("19.txt")?;
    let scan = |x, y| vm.clone().run(vec![x, y]).unwrap()[0] == 1;

    let mut left = 0;
    let mut bottom = SIZE - 1;

    loop {
        let top = bottom - SIZE + 1;

        if !scan(left, bottom) {
            left += 1;
        } else if !scan(left + SIZE - 1, top) {
            bottom += 1;
        } else {
            return Ok(println!("{}", left * 10000 + top));
        }
    }
}
