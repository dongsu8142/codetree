use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<i8>().unwrap();
    let b = iter.next().unwrap().parse::<i8>().unwrap();

    if a > 0 {
        for _ in 0..b {
            print!("{a}");
        }
    } else {
        println!("0");
    }
}