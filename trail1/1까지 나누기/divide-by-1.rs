use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut n: u16 = input.trim().parse().unwrap();

    for i in 1..=50 {
        n /= i;
        if n <= 1 {
            println!("{i}");
            break;
        }
    }
}