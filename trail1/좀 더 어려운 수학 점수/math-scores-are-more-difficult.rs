use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a_m = iter.next().unwrap().parse::<u8>().unwrap();
    let a_e = iter.next().unwrap().parse::<u8>().unwrap();
    let b_m = iter.next().unwrap().parse::<u8>().unwrap();
    let b_e = iter.next().unwrap().parse::<u8>().unwrap();

    if a_m > b_m {
        println!("A");
    } else if a_m < b_m {
        println!("B");
    } else if a_e > b_e {
        println!("A");
    } else if a_e < b_e {
        println!("B");
    }
}