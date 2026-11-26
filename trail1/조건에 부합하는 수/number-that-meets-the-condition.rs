use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a: u8 = input.trim().parse().unwrap();

    for i in 1..=a {
        if (i % 2 == 0 && i % 4 != 0) || (i / 8) % 2 == 0 || (i % 7) < 4 {
            continue
        }
        print!("{i} ");
    }
}