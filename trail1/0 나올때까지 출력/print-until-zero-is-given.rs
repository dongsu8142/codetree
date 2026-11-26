use std::io;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let n = input.trim();
        if n == "0" {break}
        println!("{n}");
        input.clear();
    }
}