use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();
    let mut cnt = 0u16;

    for i in 1..=n {
        if i % 2 == 0 || i % 3 == 0 || i % 5 == 0 {
            continue
        }
        cnt += 1;
    }

    println!("{cnt}")
}