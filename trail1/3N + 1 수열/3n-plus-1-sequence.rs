use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut n: u16 = input.trim().parse().unwrap();
    let mut cnt: usize = 0;
    loop {
        if n == 1 {
            println!("{cnt}");
            break;
        } else if n % 2 == 0 {
            n /= 2;
            cnt += 1;
        } else {
            n = n * 3 + 1;
            cnt += 1;
        }
    }
}