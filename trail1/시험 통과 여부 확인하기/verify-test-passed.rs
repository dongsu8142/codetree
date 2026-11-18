use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();
    if n >= 80 {
        println!("pass");
    } else {
        println!("{} more score", 80 - n);
    }
}