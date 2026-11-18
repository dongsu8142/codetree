use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a_a = iter.next().unwrap().parse::<u8>().unwrap();
    let a_g = iter.next().unwrap();
    let b_a = iter.next().unwrap().parse::<u8>().unwrap();
    let b_g = iter.next().unwrap();

    println!("{}", ((a_a >= 19 && a_g == "M") || (b_a >= 19 && b_g == "M")) as u8);
}