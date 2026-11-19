use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut a: u8 = iter.next().unwrap().parse().unwrap();
    let b: u8 = iter.next().unwrap().parse().unwrap();
    
    while a <= b {
        print!("{a} ");
        if a % 2 == 0 {
            a += 3;
        } else {
            a *= 2;
        }
    }
}