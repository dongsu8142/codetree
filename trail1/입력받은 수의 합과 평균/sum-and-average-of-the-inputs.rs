use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: u8 = iter.next().unwrap().parse().unwrap();
    let mut sum = 0;

    for _ in 0..n {
        let m: u16 = iter.next().unwrap().parse().unwrap();
        sum += m;
    }

    println!("{sum} {:.1}", sum as f32 / n as f32)
}