use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<i16>().unwrap());
    let mut sum = 0;
    let mut cnt: u8 = 0;
    for _ in 0..10 {
        let n = iter.next().unwrap();
        if n >= 0 && n <= 200 {
            sum += n;
            cnt += 1;
        }
    }
    println!("{sum} {:.1}", sum as f32 / cnt as f32)
}