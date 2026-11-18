use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let mut num: u8 = 0;

    for i in 1..=3 {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap().parse::<u8>().unwrap();
        if (a == "Y" && b >= 37) {
            num += 1;
        }
    }

    println!("{}", if num >= 2 {"E"} else {"N"})
}