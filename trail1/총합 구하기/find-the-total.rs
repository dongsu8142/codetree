use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<u16>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let mut sum = 0;

    for i in a..=b {
        if i % 6 == 0 && i % 8 != 0 {
            sum += i;
        }
    }

    println!("{sum}");
}