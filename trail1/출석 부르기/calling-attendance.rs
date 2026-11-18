use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let number = input.trim().parse::<u8>().unwrap();

    let name = if number == 1 
    {"John"} 
    else if number == 2 
    {"Tom"} 
    else if number == 3
    {"Paul"}
    else
    {"Vacancy"};

    println!("{name}");
}