use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let mut a: u16 = iter.next().unwrap().parse().unwrap();
    let mut b: u16 = iter.next().unwrap().parse().unwrap();

    (a, b) = (b, a);

    println!("{a} {b}");
}