use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u16>().unwrap();
    println!("{}", (a % 3 == 0 || a % 5 == 0) as u8);
}