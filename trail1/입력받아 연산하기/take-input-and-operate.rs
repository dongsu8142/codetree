use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u16>().unwrap();

    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let b = input.trim().parse::<u16>().unwrap();
    println!("{}\n{}", a + 87, b % 10);
}