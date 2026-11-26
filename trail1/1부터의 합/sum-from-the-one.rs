use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
        if sum >= n {
            println!("{i}");
            break
        }
    }
}