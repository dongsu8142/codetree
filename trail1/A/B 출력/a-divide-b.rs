use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let a = iter.next().unwrap().parse::<u16>().unwrap();
    let b = iter.next().unwrap().parse::<u16>().unwrap();

    let mut tmp = a % b;
    print!("{}.", a / b);
    for _ in 0..20 {
        tmp *= 10;
        print!("{}", tmp / b);
        tmp %= b;
    }
}