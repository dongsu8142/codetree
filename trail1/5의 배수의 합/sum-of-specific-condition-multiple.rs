use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    let (start, end) = (a.min(b), a.max(b));
    let mut sum = 0u16;

    for i in start..=end {
        if i % 5 == 0 {
            sum += i as u16;
        }
    }

    println!("{sum}");
}