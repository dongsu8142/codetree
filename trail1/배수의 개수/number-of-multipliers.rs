use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let mut cnt_t: u8 = 0;
    let mut cnt_f: u8 = 0;

    for _ in 0..10 {
        let n: u8 = iter.next().unwrap().parse().unwrap();
        if n % 3 == 0 {
            cnt_t += 1
        }
        if n % 5 == 0 {
            cnt_f += 1
        }
    }
    println!("{cnt_t} {cnt_f}");
}