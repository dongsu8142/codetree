use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();

    println!("{}", (n % 2 == 1 && n % 3 == 0) || n % 10 == 0);
}