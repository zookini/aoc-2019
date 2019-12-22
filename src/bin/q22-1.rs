use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = input("22.txt")?;
    let mut deck = Deck::new(0..10007);

    deck.run(&input);
    Ok(println!("{:?}", deck.cards.iter().position(|&c| c == 2019)))
}

struct Deck {
    cards: Vec<u32>,
}

impl Deck {
    fn new(cards: impl IntoIterator<Item = u32>) -> Deck {
        Deck {
            cards: cards.into_iter().collect(),
        }
    }

    fn run(&mut self, techniques: &str) {
        for line in techniques.lines() {
            let (end, start) = line.rsplitn(2, ' ').collect_tuple().unwrap();

            match (start, end.parse()) {
                ("cut", Ok(n)) => self.cut(n),
                ("deal with increment", Ok(n)) => self.deal_with_increment(n as usize),
                ("deal into new", Err(_)) => self.deal_into_new_stack(),
                _ => unreachable!(),
            }
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.cards.reverse();
    }

    fn cut(&mut self, n: i32) {
        if n >= 0 {
            self.cards.rotate_left(n as usize);
        } else {
            self.cards.rotate_right(-n as usize);
        }
    }

    fn deal_with_increment(&mut self, n: usize) {
        let mut new = vec![0; self.cards.len()];

        for (i, card) in self.cards.iter().enumerate() {
            new[(i * n) % self.cards.len()] = *card;
        }

        self.cards = new;
    }
}
