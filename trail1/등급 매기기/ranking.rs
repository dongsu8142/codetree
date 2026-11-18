use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let score = input.trim().parse::<u8>().unwrap();

    let class = if score >= 90
    {"A"} 
    else if score >= 80 
    {"B"} 
    else if score >= 70
    {"C"}
    else if score >= 60
    {"D"}
    else
    {"F"};

    println!("{class}");
}