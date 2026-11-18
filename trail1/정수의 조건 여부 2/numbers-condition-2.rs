use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut a = input.trim().parse::<u16>().unwrap();

    if a == 5 {
        println!("A");
    } else if a % 2 == 0 {
        println!("B")
    }
}