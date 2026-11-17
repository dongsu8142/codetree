use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let a: u8 = iter.next().unwrap().parse().unwrap();
    let b: u8 = iter.next().unwrap().parse().unwrap();

    println!("{}", a + b);
}