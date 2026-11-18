use std::io::{self, Read};

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let gender = iter.next().unwrap().parse::<u8>().unwrap();
    let age = iter.next().unwrap().parse::<u8>().unwrap();

    if gender == 0 {
        println!("{}", if age >= 19 {"MAN"} else {"BOY"});
    } else {
        println!("{}", if age >= 19 {"WOMAN"} else {"GIRL"});
    }
}