use std::io;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let n: u8 = input.trim().parse().unwrap();
        if n == 25 {
            println!("Good");
            break;
        } else if n > 25 {
            println!("Lower");
        } else {
            println!("Higher")
        }
        input.clear()
    }
}