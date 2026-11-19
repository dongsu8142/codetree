use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut a = iter.next().unwrap().parse::<u8>().unwrap();
    let n = iter.next().unwrap().parse::<u8>().unwrap();

    for _ in 0..n {
        a += n;
        println!("{a}");
    }
}