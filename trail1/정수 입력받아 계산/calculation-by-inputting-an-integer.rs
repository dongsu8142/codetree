use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let num = a.trim().parse::<u8>().unwrap();

    println!("{}", num * 2 + 3);
}