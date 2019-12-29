use aoc::*;
use futures::channel::mpsc;
use futures::prelude::*;
use itertools::{izip, Itertools};

#[tokio::main]
async fn main() -> Result<()> {
    let image = Computer::load("7.txt")?;

    let signal = (5..10)
        .permutations(5)
        .map(|phases| futures::executor::block_on(amplify(&image, phases)))
        .max();

    Ok(println!("{:?}", signal))
}

async fn amplify(image: &Computer, phases: Vec<i64>) -> i64 {
    let last = phases.len() - 1;
    let (txs, mut rxs): (Vec<_>, Vec<_>) = phases.iter().map(|_| mpsc::channel(2)).unzip();
    let mut result = mpsc::unbounded();

    rxs.rotate_right(1);

    future::join_all(
        izip!(0..phases.len(), phases, txs, rxs).map(|(i, phase, tx, rx)| {
            let mut vm = image.clone();
            let mut tx = tx.fanout(result.0.clone());

            tokio::spawn(async move {
                tx.send(phase).await.unwrap();

                if i == last {
                    tx.send(0).await.unwrap();
                }

                vm.interact(rx, tx).await.unwrap();
            })
        }),
    )
    .await;

    result.0.close().await.unwrap();
    result.1.fold(0, |_, last| async move { last }).await
}
