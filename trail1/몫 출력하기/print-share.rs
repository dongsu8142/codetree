use std::io;

fn main() {
    let mut input = String::new();
    let mut cnt = 0u8;

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let n: u8 = input.trim().parse().unwrap();
        if n % 2 == 0 {
            println!("{}", n / 2);
            cnt += 1;
        }
        if cnt == 3 {break}
        input.clear()
    }
}