use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<f32>().unwrap();
    let b = iter.next().unwrap().parse::<f32>().unwrap();
    println!("{:.2}", (a + b) / (a - b))
}