use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<i16>().unwrap();

    if a >= 113 {
        println!("1");
    } else {
        println!("0");
    }
}