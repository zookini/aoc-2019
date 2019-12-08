fn main() {
    let start = 353_096;
    let end = 843_212;

    println!("{}", (start..=end).filter(|&i| validate(i)).count())
}

fn validate(i: i32) -> bool {
    let mut head = i / 10;
    let mut tail = i % 10;
    let mut adjacent = false;

    loop {
        let prev = head % 10;
        if tail < prev {
            return false;
        }
        if tail == prev {
            adjacent = true;
        }
        if head == 0 {
            return adjacent;
        }

        head /= 10;
        tail = prev;
    }
}
