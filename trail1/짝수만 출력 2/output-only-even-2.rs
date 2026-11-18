use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut b = iter.next().unwrap().parse::<u8>().unwrap();
    let a = iter.next().unwrap().parse::<u8>().unwrap();

    while b >= a {
        print!("{b} ");
        b -= 2;
    }
}