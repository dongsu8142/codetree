use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split("-");
    let num1 = iter.next().unwrap();
    let num2 = iter.next().unwrap();
    let num3 = iter.next().unwrap();
    println!("{num1}-{num3}-{num2}")
}