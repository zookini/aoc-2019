use aoc::*;
use std::collections::HashSet;
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

fn router(messages: Receiver<(usize, (i64, i64))>, mut nics: Vec<Sender<i64>>) -> Result<i64> {
    let mut nat: Option<(i64, i64)> = None;
    let mut seen = HashSet::new();

    loop {
        match messages.recv_timeout(time::Duration::from_millis(5)) {
            Ok((destination, packet)) => {
                println!("Received packet {:?} for {}", packet, destination);

                if destination == 255 {
                    nat = Some(packet);
                } else {
                    let nic = &mut nics[destination];
                    nic.send(packet.0)?;
                    nic.send(packet.1)?;
                }
            }
            Err(_) => {
                if let Some((x, y)) = nat {
                    if seen.contains(&y) {
                        return Ok(y);
                    } else {
                        seen.insert(y);

                        nics[0].send(x)?;
                        nics[0].send(y)?;
                    }
                }
            }
        }
    }
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
