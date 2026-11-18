use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    let c: i32 = iter.next().unwrap().parse().unwrap();

    let mid = if (b <= a && a <= c) || (c <= a && a <= b) {
        a
    } else if (a <= b && b <= c) || (c <= b && b <= a) {
        b
    } else {
        c
    };

    println!("{}", mid);
}