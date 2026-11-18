use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let score = input.trim().parse::<u8>().unwrap();
    
    let pass = if score == 100 {"pass"} else {"failure"};
    println!("{pass}");
}