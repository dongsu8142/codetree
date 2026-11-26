use std::io;

fn main() {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let width: u16 = iter.next().unwrap().parse().unwrap();
        let height: u16 = iter.next().unwrap().parse().unwrap();
        let char = iter.next().unwrap();
        println!("{}", width * height);
        if char == "C" {break}
        input.clear();
    }
}