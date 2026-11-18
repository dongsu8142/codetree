use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let money = input.trim().parse::<u16>().unwrap();

    let object = if money >= 3000 {"book"} 
    else if money >= 1000 {"mask"} 
    else if money >= 500 {"pen"}
    else {"no"};

    println!("{object}");
}