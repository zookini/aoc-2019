use aoc::*;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = input("22.txt")?;

    println!("{}", solve2(&input, 119315717514047, 101741582076661, 2020));
    Ok(())
}

fn solve(techniques: &str, cards: i128, times: i128, position: i128) -> i128 {
    let mut offset: i128 = 0;
    let mut increment: i128 = 1;

    let inverse = |i| mod_exp(i, cards - 2, cards);

    for line in techniques.lines() {
        let (end, start) = line.rsplitn(2, ' ').collect_tuple().unwrap();

        match (start, end.parse()) {
            ("cut", Ok(n)) => offset = (offset + increment * n as i128).rem_euclid(cards),
            ("deal with increment", Ok(n)) => {
                increment = (increment * inverse(n)).rem_euclid(cards)
            }
            ("deal into new", Err(_)) => {
                increment = (increment * -1).rem_euclid(cards);
                offset = (offset + increment).rem_euclid(cards);
            }
            _ => unreachable!(),
        }
    }

    let increments = mod_exp(increment, times, cards);
    let offsets =
        (offset * (1 - increments) * inverse((1 - increment).rem_euclid(cards))).rem_euclid(cards);

    (offsets + position * increments) % cards
}

fn linearise(techniques: &str, cards: i128) -> (i128, i128) {
    techniques.lines().rev().fold((1, 0), |(a, b), technique| {
        let (end, start) = technique.rsplitn(2, ' ').collect_tuple().unwrap();

        let (a, b) = match (start, end.parse()) {
            ("cut", Ok(n)) => (a, b + n),
            ("deal into new", _) => (-a, -b - 1),
            ("deal with increment", Ok(n)) => {
                let inverse = mod_exp(n, cards - 2, cards);
                (a * inverse, b * inverse)
            }
            _ => unreachable!(),
        };

        (a % cards, b % cards)
    })
}

fn solve2(techniques: &str, cards: i128, times: i128, position: i128) -> i128 {
    let inverse = |i| mod_exp(i, cards - 2, cards);

    let (a, b) = linearise(techniques, cards);

    let term1 = position * mod_exp(a, times, cards) % cards;
    let tmp = (mod_exp(a, times, cards) - 1) * inverse(a - 1) % cards;
    let term2 = b * tmp % cards;
    (term1 + term2).rem_euclid(cards)
}

fn mod_exp(base: i128, exponent: i128, modulus: i128) -> i128 {
    let mut base = base % modulus;
    let mut exponent = exponent;

    let mut result = 1;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }

        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    result
}
