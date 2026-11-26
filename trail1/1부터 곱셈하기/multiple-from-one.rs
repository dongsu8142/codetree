use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();
    let mut prod = 1u32;

    for i in 1..=10 {
        prod *= i;
        if prod >= n {
            println!("{i}");
            break;
        }
    }
}