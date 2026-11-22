use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let mut cnt = 0;
    for _ in 0..5{
        if iter.next().unwrap().parse::<u16>().unwrap() % 2 == 0 {
            cnt += 1;
        }
    }
    
    println!("{cnt}");
}