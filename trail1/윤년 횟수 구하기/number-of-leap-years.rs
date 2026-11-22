use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();

    let mut cnt = 0;

    for i in 1..=n {
        if (i % 4 == 0 && i % 100 != 0) || (i % 400 == 0) {
            cnt += 1;
        }
    }

    println!("{cnt}");
}