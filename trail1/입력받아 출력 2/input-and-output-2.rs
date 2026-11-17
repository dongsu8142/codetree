use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split("-");
    println!("{}{}", iter.next().unwrap(), iter.next().unwrap());
}