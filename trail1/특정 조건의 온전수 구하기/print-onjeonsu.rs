use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    for i in 1..=n {
        if i % 2 == 0 || (i % 5 == 0 && i % 10 != 0) || (i % 3 == 0 && i % 9 != 0) {
            continue
        }
        print!("{i} ");
    }
}