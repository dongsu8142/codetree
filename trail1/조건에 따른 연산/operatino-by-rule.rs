use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut n: u16 = input.trim().parse().unwrap();
    let mut cnt = 0usize;

    loop {
        if n % 2 == 0 {
            n = n * 3 + 1;
        } else {
            n = n * 2 + 2;
        }
        cnt += 1;
        if n >= 1000 {
            println!("{cnt}");
            break;
        }
    }
}