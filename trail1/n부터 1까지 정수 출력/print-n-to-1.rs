use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut n = input.trim().parse::<u8>().unwrap();

    while n >= 1 {
        print!("{n} ");
        n -= 1;
    }
}