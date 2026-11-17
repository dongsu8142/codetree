use std::io;

fn main() {
    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).unwrap();
    io::stdin().read_line(&mut t).unwrap();

    println!("{}", t.trim());
    println!("{}", s.trim());
}