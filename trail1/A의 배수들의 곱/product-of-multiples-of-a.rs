use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<u16>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let mut prod = 1;

    for i in a..=b {
        if i % a == 0 {
            prod *= i;
        }
    }

    println!("{prod}")
}