use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let h = iter.next().unwrap().parse::<u32>().unwrap();
    let w = iter.next().unwrap().parse::<u32>().unwrap();

    let b = (10000 * w) / (h * h);
    println!("{b}");
    if b >= 25 {
        println!("Obesity");
    }
}