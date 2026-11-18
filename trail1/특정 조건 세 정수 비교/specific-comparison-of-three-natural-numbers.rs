use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<u8>().unwrap();
    let b = iter.next().unwrap().parse::<u8>().unwrap();
    let c = iter.next().unwrap().parse::<u8>().unwrap();

    let min = a.min(b).min(c);

    println!("{} {}", (a == min) as u8, (a == b && a == c) as u8);
}