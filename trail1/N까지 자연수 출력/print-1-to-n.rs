use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();
    let mut i = 1;
    while i <= n {
        print!("{i} ");
        i += 1;
    }
}