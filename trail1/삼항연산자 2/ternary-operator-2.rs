use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u8>().unwrap();

    let b = if a == 1 {"t"} else {"f"};
    println!("{b}");
}