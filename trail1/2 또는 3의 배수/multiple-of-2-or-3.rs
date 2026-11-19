use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();

    for i in 1..=n {
        if i % 2 == 0 || i % 3 == 0 {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }
}