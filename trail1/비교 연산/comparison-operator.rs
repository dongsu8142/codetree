use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<u16>().unwrap();
    let b = iter.next().unwrap().parse::<u16>().unwrap();

    println!("{:b}", (a >= b) as u8);
    println!("{:b}", (a > b) as u8);
    println!("{:b}", (a <= b) as u8);
    println!("{:b}", (a < b) as u8);
    println!("{:b}", (a == b) as u8);
    println!("{:b}", (a != b) as u8);
}