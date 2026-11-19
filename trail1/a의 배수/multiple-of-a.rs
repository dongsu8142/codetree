use std::io::{self, Read};

fn main(){
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<u8>().unwrap();
    let a = iter.next().unwrap().parse::<u8>().unwrap();
    let mut i = 1;
    while i <= n {
        if i % a == 0 {
            println!("1");
        } else {
            println!("0");
        }
        i += 1
    }
}