use aoc::*;
use itertools::{izip, Itertools};
use std::sync::mpsc;

fn main() -> Result<()> {
    let image = Computer::load("7.txt")?;

    let signal = (5..10)
        .permutations(5)
        .map(|phases| amplify(&image, phases))
        .max();

    Ok(println!("{:?}", signal))
}

fn amplify(image: &Computer, phases: Vec<i64>) -> i64 {
    let last = phases.len() - 1;
    let (txs, mut rxs): (Vec<_>, Vec<_>) = phases.iter().map(|_| mpsc::channel()).unzip();
    let result = mpsc::channel();

    rxs.rotate_right(1);

    let handles: Vec<_> = izip!(0..phases.len(), phases, txs, rxs)
        .map(|(i, phase, tx, rx)| {
            let mut vm = image.clone();
            let mut tx = Fanout(result.0.clone(), tx);

            std::thread::spawn(move || {
                tx.send(phase).unwrap();

                if i == last {
                    tx.send(0).unwrap();
                }

                let _ = vm.interact(rx, tx);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    result.1.try_iter().last().unwrap()
}

struct Fanout<A, B>(A, B);

impl<A, B, T> Sink<T> for Fanout<A, B>
where
    A: Sink<T>,
    B: Sink<T>,
    T: Copy,
{
    fn send(&mut self, item: T) -> Result<()> {
        self.0.send(item)?;
        self.1.send(item)?;
        Ok(())
    }
}
