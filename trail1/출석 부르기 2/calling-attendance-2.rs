use std::io;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim();
        if n == "1" {
            println!("John");
        } else if n == "2" {
            println!("Tom");
        } else if n == "3" {
            println!("Paul");
        } else if n == "4" {
            println!("Sam");
        } else {
            println!("Vacancy");
            break;
        }
        input.clear();
    }
}