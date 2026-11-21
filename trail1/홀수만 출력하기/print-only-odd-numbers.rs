use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: u8 = iter.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let a: u8 = iter.next().unwrap().parse().unwrap();
        if a % 3 == 0 && a % 2 != 0 {
            println!("{a}");
        }
    }
}