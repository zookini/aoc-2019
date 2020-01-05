use aoc::*;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time;

fn main() -> Result<()> {
    let image = Computer::parse("23.txt")?;
    let rio = mpsc::channel();
    let (txs, rxs): (Vec<_>, Vec<_>) = (0..50).map(|_| mpsc::channel()).unzip();

    for (i, rx) in rxs.into_iter().enumerate() {
        let mut computer = Computer::new(image.clone());
        let adapter = Adapter::new(rio.0.clone());

        txs[i].send(i as i64)?;
        thread::spawn(move || computer.interact(nin(rx), adapter).unwrap());
    }

    Ok(println!("{:?}", router(rio.1, txs)))
}

fn nin(rx: Receiver<i64>) -> impl Iterator<Item = i64> {
    std::iter::from_fn(move || match rx.try_recv() {
        Ok(input) => Some(input),
        Err(TryRecvError::Disconnected) => None,
        Err(TryRecvError::Empty) => {
            thread::sleep(time::Duration::from_millis(1));
            Some(-1)
        }
    })
}

fn router(
    messages: impl IntoIterator<Item = (usize, (i64, i64))>,
    mut nics: Vec<Sender<i64>>,
) -> Result<i64> {
    for (destination, (x, y)) in messages {
        if destination == 255 {
            return Ok(y);
        } else {
            let nic = &mut nics[destination];
            nic.send(x)?;
            nic.send(y)?;
        }
    }

    unreachable!()
}

struct Adapter {
    buffer: Vec<i64>,
    router: Sender<(usize, (i64, i64))>,
}

impl Adapter {
    fn new(router: Sender<(usize, (i64, i64))>) -> Self {
        Adapter {
            buffer: Vec::with_capacity(3),
            router,
        }
    }
}

impl Sink<i64> for Adapter {
    fn send(&mut self, item: i64) -> Result<()> {
        self.buffer.push(item);

        if let &[destination, x, y] = &self.buffer[..] {
            self.router.send((destination as usize, (x, y)))?;
            self.buffer.clear()
        }

        Ok(())
    }
}
