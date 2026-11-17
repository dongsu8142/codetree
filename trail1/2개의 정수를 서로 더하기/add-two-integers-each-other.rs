use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let mut a = iter.next().unwrap().parse::<u16>().unwrap();
    let mut b = iter.next().unwrap().parse::<u16>().unwrap();
    a += b;
    b += a;

    println!("{a} {b}")
}