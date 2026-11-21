use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u8 = input.trim().parse().unwrap();

    for i in 1..=n {
        let s = i.to_string();
        if i % 3 == 0 || s.contains('3') || s.contains('6') || s.contains('9') {
            print!("0 ");
        } else {
            print!("{} ", i);
        }
    }
}