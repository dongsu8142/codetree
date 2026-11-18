use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let m = input.trim();

    for i in 0..8 {
        print!("{m}")
    }
}