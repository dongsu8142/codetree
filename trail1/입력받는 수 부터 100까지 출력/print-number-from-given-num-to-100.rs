use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n = input.trim().parse::<u8>().unwrap();

    for i in n..=100{
        print!("{} ", i)
    }
}