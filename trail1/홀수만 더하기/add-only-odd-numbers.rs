use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|n| n.parse::<u8>().unwrap());
    let n = iter.next().unwrap();
    
    let mut sum = 0u16;

    for i in 0..n {
        let m = iter.next().unwrap();
        if m % 2 != 0 && m % 3 == 0 {
            sum += m as u16;
        }
    }

    println!("{sum}");
}