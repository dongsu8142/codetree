use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u16>().unwrap();
    
    let m = if n >= 3000 {"book"} else if n >= 1000 {"mask"} else {"no"};
    println!("{m}");
}