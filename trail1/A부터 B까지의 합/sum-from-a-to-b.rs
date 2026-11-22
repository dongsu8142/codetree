use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|n| n.parse::<u16>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();

    let mut sum = 0;
    for i in a..=b {
        sum += i;
    }
    println!("{sum}");
}