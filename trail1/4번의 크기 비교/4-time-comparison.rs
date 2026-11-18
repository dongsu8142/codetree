use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<u8>().unwrap();
    let b = iter.next().unwrap().parse::<u8>().unwrap();
    let c = iter.next().unwrap().parse::<u8>().unwrap();
    let d = iter.next().unwrap().parse::<u8>().unwrap();
    let e = iter.next().unwrap().parse::<u8>().unwrap();

    println!("{:b}", (a > b) as u8);
    println!("{:b}", (a > c) as u8);
    println!("{:b}", (a > d) as u8);
    println!("{:b}", (a > e) as u8);
}