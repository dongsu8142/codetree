use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let m = iter.next().unwrap().parse::<u8>().unwrap();
    let f = iter.next().unwrap().parse::<u8>().unwrap();

    if m < 90 {
        println!("0");
    } else if f >= 95 {
        println!("100000");
    } else if f >= 90 {
        println!("50000");
    } else {
        println!("0")
    }
}