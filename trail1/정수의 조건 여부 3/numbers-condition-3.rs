use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let a = input.trim().parse::<u16>().unwrap();
    println!("{}", if a % 13 == 0 || a % 19 == 0 {"True"} else {"False"});
}