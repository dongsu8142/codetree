use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut a = input.trim().parse::<u8>().unwrap();

    if a % 2 == 0 {
        a /= 2
    };

    if a % 2 == 1 {
        a = (a + 1) / 2
    }

    println!("{a}");
}