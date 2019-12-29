use aoc::*;
use futures::prelude::*;
use std::collections::HashMap;
use std::time::{self, Duration};
use tokio;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[tokio::main]
async fn main() -> Result<()> {
    let image = Computer::parse("23.txt")?;

    let (nics, mut io): (Vec<_>, Vec<_>) = (0..50)
        .map(|_| {
            let (input, output) = (mpsc::channel(10), mpsc::channel(3));
            let nic = Computer::new(image.clone(), input.1, output.0);

            (nic, (output.1, input.0))
        })
        .unzip();

    for (i, mut nic) in nics.into_iter().enumerate() {
        io[i].1.send(i as i64).await.unwrap();

        tokio::spawn(async move {
            nic.run().await.unwrap();
        });
    }

    Ok(println!("{}", router(io).await?))
}

async fn router(io: Vec<(Receiver<i64>, Sender<i64>)>) -> Result<i64> {
    let (inputs, mut outputs): (Vec<_>, Vec<_>) = io
        .into_iter()
        .map(|(i, o)| (Box::pin(messages(i)), o))
        .unzip();

    let mut inputs = stream::select_all(inputs);
    let mut nat: Option<(i64, i64)> = None;
    let mut counts: HashMap<i64, u8> = HashMap::new();

    loop {
        match tokio::time::timeout(Duration::from_millis(5), inputs.next()).await {
            Ok(Some((destination, packet))) => {
                println!("Received packet {:?} for {}", packet, destination);

                if destination == 255 {
                    nat = Some(packet);
                } else {
                    let output = &mut outputs[destination as usize];
                    output.send(packet.0).await?;
                    output.send(packet.1).await?;
                }
            }
            Err(_) => {
                if let Some((x, y)) = nat {
                    let count = counts.entry(y).or_insert(1);
                    if *count >= 2 {
                        return Ok(y);
                    } else {
                        *count += 1;

                        outputs[0].send(x).await?;
                        outputs[0].send(y).await?;
                    }
                }
            }
            _ => (),
        }
    }
}

fn messages(rx: Receiver<i64>) -> impl Stream<Item = (i64, (i64, i64))> {
    stream::unfold(rx, |mut rx| {
        async {
            match (rx.recv().await, rx.recv().await, rx.recv().await) {
                (Some(destination), Some(x), Some(y)) => Some(((destination, (x, y)), rx)),
                _ => None,
            }
        }
    })
}

struct Computer {
    base: usize,
    mem: Vec<i64>,
    ip: usize,
    input: Receiver<i64>,
    output: Sender<i64>,
}

impl Computer {
    fn new(mut mem: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Self {
        mem.resize(10 * 1024, 0);

        Self {
            base: 0,
            mem,
            ip: 0,
            input,
            output,
        }
    }

    fn parse(filename: &str) -> Result<Vec<i64>> {
        Ok(input(filename)?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect())
    }

    const OP_SIZE: &'static [usize] = &[0, 4, 4, 2, 2, 3, 3, 4, 4, 2];

    async fn run(&mut self) -> Result<()> {
        loop {
            let op = (self.mem[self.ip] % 100) as usize;

            match op {
                1 => *self.at(3) = *self.at(1) + *self.at(2),
                2 => *self.at(3) = *self.at(1) * *self.at(2),
                3 => match self.input.try_recv() {
                    Ok(input) => {
                        *self.at(1) = input;
                    }
                    Err(mpsc::error::TryRecvError::Empty) => {
                        *self.at(1) = -1;
                        tokio::time::delay_for(time::Duration::from_millis(1)).await;
                    }
                    Err(e) => return Err(e.into()),
                },
                4 => {
                    let output = *self.at(1);
                    self.output.send(output).await?;
                }
                5 => {
                    if *self.at(1) != 0 {
                        self.ip = *self.at(2) as usize;
                        continue;
                    }
                }
                6 => {
                    if *self.at(1) == 0 {
                        self.ip = *self.at(2) as usize;
                        continue;
                    }
                }
                7 => *self.at(3) = if *self.at(1) < *self.at(2) { 1 } else { 0 },
                8 => *self.at(3) = if *self.at(1) == *self.at(2) { 1 } else { 0 },
                9 => self.base = (self.base as i64 + *self.at(1)) as usize,
                99 => return Ok(()),
                _ => unreachable!(),
            }

            self.ip += Self::OP_SIZE[op];
        }
    }

    fn at(&mut self, parameter: usize) -> &mut i64 {
        let mode = self.mem[self.ip] / 10i64.pow(parameter as u32 + 1) % 10;
        let parameter = self.ip + parameter;

        let address = match mode {
            0 => self.mem[parameter] as usize,
            1 => parameter,
            2 => (self.base as i64 + self.mem[parameter]) as usize,
            _ => unreachable!(),
        };

        &mut self.mem[address]
    }
}
