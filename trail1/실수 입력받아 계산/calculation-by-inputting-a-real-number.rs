use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a: f32 = input.trim().parse().unwrap();

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let b: f32 = input.trim().parse().unwrap();

    println!("{:.2}", a + b);
}