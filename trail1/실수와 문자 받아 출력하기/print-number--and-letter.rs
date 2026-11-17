use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input.trim());
    
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    println!("{:.2}", input.trim().parse::<f32>().unwrap());

    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    println!("{:.2}", input.trim().parse::<f32>().unwrap());
}