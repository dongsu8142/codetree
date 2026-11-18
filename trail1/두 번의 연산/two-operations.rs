use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut a = input.trim().parse::<u8>().unwrap();

    if a % 2 == 1 {
        a += 3
    };

    if a % 3 == 0 {
        a /= 3
    }

    println!("{a}");
}