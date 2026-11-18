use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<u8>().unwrap();

    println!("{}", if a % 3 == 0 {"YES"} else {"NO"});
    println!("{}", if a % 5 == 0 {"YES"} else {"NO"})
}