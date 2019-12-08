fn main() {
    let start = 353_096;
    let end = 843_212;

    println!("{}", (start..=end).filter(|&i| validate(i)).count())
}

fn validate(i: i32) -> bool {
    let mut head = i / 10;
    let mut tail = i % 10;
    let mut pair = false;
    let mut combo = 1;

    loop {
        let prev = head % 10;

        if tail < prev {
            return false;
        }

        if tail == prev {
            combo += 1;
        } else {
            if combo == 2 {
                pair = true;
            }
            combo = 1;
        }

        if head == 0 {
            return pair;
        }

        head /= 10;
        tail = prev;
    }
}
