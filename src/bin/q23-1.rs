use aoc::*;
use futures::channel::mpsc::{self, Receiver, Sender};
use futures::prelude::*;
use std::time;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let image = Computer::parse("23.txt")?;

    let (nio, mut rio): (Vec<_>, Vec<_>) = (0..50)
        .map(|_| {
            let (input, output) = (mpsc::channel(100), mpsc::channel(3));
            ((nin(input.1), output.0), (output.1, input.0))
        })
        .unzip();

    for (i, (input, output)) in nio.into_iter().enumerate() {
        let mut computer = Computer::new(image.clone());
        rio[i].1.send(i as i64).await?;

        tokio::spawn(async move {
            computer.interact(input, output).await.unwrap();
        });
    }

    Ok(println!("{:?}", router(rio).await))
}

fn nin(rx: Receiver<i64>) -> impl Stream<Item = i64> {
    stream::unfold(rx, |mut rx| {
        async {
            match rx.next().now_or_never() {
                Some(Some(input)) => Some((input, rx)),
                None => {
                    tokio::time::delay_for(time::Duration::from_millis(1)).await;
                    Some((-1, rx))
                }
                _ => None,
            }
        }
        .boxed()
    })
}

async fn router(io: Vec<(Receiver<i64>, Sender<i64>)>) -> Result<i64> {
    let (inputs, mut outputs): (Vec<_>, Vec<_>) = io
        .into_iter()
        .map(|(i, o)| (i.chunks(3).map(|v| (v[0], (v[1], v[2]))), o))
        .unzip();

    let mut inputs = stream::select_all(inputs);

    while let Some((destination, packet)) = inputs.next().await {
        if destination == 255 {
            return Ok(packet.1);
        } else {
            let output = &mut outputs[destination as usize];
            output.send(packet.0).await?;
            output.send(packet.1).await?;
        }
    }

    unreachable!()
}
