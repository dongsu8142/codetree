use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u16>().unwrap();

    println!("{}", n.pow(2));
    if n < 5 {
        println!("tiny");
    }
}