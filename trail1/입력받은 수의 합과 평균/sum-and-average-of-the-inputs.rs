use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: u8 = iter.next().unwrap().parse().unwrap();
    let sum: u16 = iter.map(|s| s.parse::<u16>().unwrap()).sum();

    println!("{sum} {:.1}", sum as f32 / n as f32)
}