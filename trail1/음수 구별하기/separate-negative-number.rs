use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i8>().unwrap();

    println!("{n}");
    if n < 0 {
        println!("minus");
    }
}