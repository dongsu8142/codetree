use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let m = input.trim().parse::<u8>().unwrap();

    let message = if m >= 12 || m <= 2 {"Winter"}
    else if m >= 9 {"Fall"}
    else if m >= 6 {"Summer"}
    else if m >= 3 {"Spring"}
    else {""};

    println!("{message}");
}