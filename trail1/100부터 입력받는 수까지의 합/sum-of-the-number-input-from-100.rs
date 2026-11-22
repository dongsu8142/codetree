use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    let mut sum = 0u16;

    for i in n..=100 {
        sum += i as u16;
    }
    println!("{sum}");
}