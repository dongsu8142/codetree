use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u8>().unwrap();
    for i in n..=100 {
        let grade = if i >= 90 {"A"}
        else if i >= 80 {"B"}
        else if i >= 70 {"C"}
        else if i >= 60 {"D"}
        else {"F"};
        print!("{grade} ");
    }
}