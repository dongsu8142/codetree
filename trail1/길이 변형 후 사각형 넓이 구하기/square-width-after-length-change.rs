use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let w = iter.next().unwrap().parse::<u16>().unwrap() + 8;
    let h = iter.next().unwrap().parse::<u16>().unwrap() * 3;

    println!("{w}\n{h}\n{}", w * h);
}