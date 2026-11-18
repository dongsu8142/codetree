use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let y = input.trim().parse::<u16>().unwrap();
    println!("{}", (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0))
}