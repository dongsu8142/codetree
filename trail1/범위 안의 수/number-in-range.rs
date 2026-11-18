use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u8>().unwrap();
    println!("{}", if 10 <= a && a <= 20 {"yes"} else {"no"});
}