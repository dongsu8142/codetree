use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let c = iter.next().unwrap();
    let n = iter.next().unwrap().parse::<u8>().unwrap();

    for i in 0..n {
        print!("{} ", if c == "A" { i + 1 } else { n - i });
    }
}