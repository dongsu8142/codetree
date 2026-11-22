use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    let mut sum = 0u16;
    let mut cnt = 0u8;

    for i in a..=b {
        if i % 5 == 0 || i % 7 == 0 {
            sum += i as u16;
            cnt += 1;
        }
    }

    println!("{sum} {:.1}", sum as f32 / cnt as f32)
}