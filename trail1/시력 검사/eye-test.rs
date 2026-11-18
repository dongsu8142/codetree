use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<f32>().unwrap();
    let b = iter.next().unwrap().parse::<f32>().unwrap();

    if a >= 1.0 && b >= 1.0 {
        println!("High");
    } else if a >= 0.5 && b >= 0.5 {
        println!("Middle");
    } else {
        println!("Low");
    }
}