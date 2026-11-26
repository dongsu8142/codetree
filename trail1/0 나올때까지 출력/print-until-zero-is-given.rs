use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let iter = input.split_whitespace();
    for i in iter {
        if i == "0" {break}
        println!("{i}");
    }
}