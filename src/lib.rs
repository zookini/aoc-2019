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

    pub fn spawn(mut self) -> (Sender<i64>, Receiver<i64>, JoinHandle<Result<()>>) {
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
                3 => *self.at(1) = input.next().await.ok_or("No more input")?,
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

pub struct Ascii {
    pub tx: Sender<i64>,
    pub rx: Receiver<i64>,
}

impl Ascii {
    pub fn new(tx: Sender<i64>, rx: Receiver<i64>) -> Ascii {
        Ascii { tx, rx }
    }

    pub async fn line(&mut self) -> Option<String> {
        let mut s = String::new();

        loop {
            match self.rx.next().await {
                Some(i) if i == 10 => return Some(s),
                Some(i) => s.push(i as u8 as char),
                None => return None,
            }
        }
    }

    pub fn lines(&mut self) -> impl Stream<Item = String> + Unpin + '_ {
        stream::unfold(self, |me| {
            async { me.line().await.map(|s| (s, me)) }.boxed()
        })
    }

    pub async fn paragraph(&mut self) -> Option<String> {
        let mut s = String::new();

        loop {
            match self.rx.next().await {
                Some(i) if s.ends_with('\n') && i == 10 => return Some(s),
                Some(i) => s.push(i as u8 as char),
                None => return None,
            }
        }
    }

    pub async fn send(&mut self, s: &str) -> Result<()> {
        for ch in s.chars().chain(std::iter::once('\n')) {
            self.tx.send(ch as i64).await?;
        }

        Ok(())
    }
}
