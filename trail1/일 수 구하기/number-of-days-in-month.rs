use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let n = input.trim().parse::<u8>().unwrap();
    
    if (n <= 7 && n % 2 == 1) || (n > 7 && n % 2 == 0) {
        println!("31")
    } else {
        println!("{}", if n == 2 {28} else {30})
    }
}