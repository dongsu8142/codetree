use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<u8>().unwrap();
    let b = iter.next().unwrap().parse::<u8>().unwrap();
    for i in (a..=b).step_by(2) {
        print!("{i} ")
    }
}