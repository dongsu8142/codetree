use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u16>().unwrap();

    for i in 1..=5 {
        print!("{} ", n * i)
    }
}