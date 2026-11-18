use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<u8>().unwrap();
    let b = iter.next().unwrap().parse::<u8>().unwrap();
    let c = iter.next().unwrap().parse::<u8>().unwrap();

    println!("{:b}", (b > a && b < c) as u8);
}