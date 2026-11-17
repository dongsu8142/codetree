use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split(".");
    let y = iter.next().unwrap();
    let m = iter.next().unwrap();
    let d = iter.next().unwrap().trim_end();
    println!("{m}-{d}-{y}")
}