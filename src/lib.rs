use futures::channel::mpsc::{self, Receiver, Sender};
use futures::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use tokio::task::JoinHandle;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn input(name: &str) -> io::Result<String> {
    let mut s = String::new();
    File::open(Path::new("input").join(name))?.read_to_string(&mut s)?;
    Ok(s)
}

#[derive(Clone)]
pub struct Computer {
    base: usize,
    pub mem: Vec<i64>,
    pub ip: usize,
}

impl Computer {
    pub fn new(mut mem: Vec<i64>) -> Self {
        mem.resize(10 * 1024, 0);

        Self {
            base: 0,
            mem,
            ip: 0,
        }
    }

    pub fn load(filename: &str) -> Result<Self> {
        Ok(Self::new(Self::parse(filename)?))
    }

    pub fn parse(filename: &str) -> Result<Vec<i64>> {
        Ok(input(filename)?
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect())
    }

    const OP_SIZE: &'static [usize] = &[0, 4, 4, 2, 2, 3, 3, 4, 4, 2];

    pub fn run(&mut self, input: impl IntoIterator<Item = i64>) -> Result<Vec<i64>> {
        let mut output = vec![];
        futures::executor::block_on(self.interact(stream::iter(input), &mut output))?;
        Ok(output)
    }

    pub fn channelled(mut self) -> (Sender<i64>, Receiver<i64>, JoinHandle<Result<()>>) {
        let (itx, irx) = mpsc::channel(1);
        let (otx, orx) = mpsc::channel(1);

        (
            itx,
            orx,
            tokio::spawn(async move { self.interact(irx, otx).await }),
        )
    }

    pub async fn interact<I, O, E>(&mut self, mut input: I, mut output: O) -> Result<()>
    where
        I: Stream<Item = i64> + Unpin,
        O: Sink<i64, Error = E> + Unpin,
        E: std::error::Error + 'static + Send + Sync,
    {
        loop {
            let op = (self.mem[self.ip] % 100) as usize;

            match op {
                1 => *self.at(3) = *self.at(1) + *self.at(2),
                2 => *self.at(3) = *self.at(1) * *self.at(2),
                3 => *self.at(1) = input.next().await.unwrap(),
                4 => output.send(*self.at(1)).await?,
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
                99 => {
                    // Stay alive to prevent input stream dropping on others
                    tokio::time::delay_for(std::time::Duration::from_millis(1)).await;
                    return Ok(());
                }
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
