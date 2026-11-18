use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i16>().unwrap();

    let message = if n < 0 {"ice"} else if n >= 100 {"vapor"} else {"water"};
    println!("{message}")
}