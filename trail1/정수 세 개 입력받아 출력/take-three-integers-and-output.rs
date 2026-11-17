use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a: u16 = iter.next().unwrap().parse().unwrap();
    let b: u16 = iter.next().unwrap().parse().unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let c: u16 = input.trim().parse().unwrap();

    println!("{a} {b} {c}");
}