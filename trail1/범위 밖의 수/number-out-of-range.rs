use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let a = input.trim().parse::<u8>().unwrap();

    if a < 10 || a > 20 {
        println!("yes");
    } else {
        println!("no");
    }
}