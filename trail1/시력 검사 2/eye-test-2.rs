use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a = input.trim().parse::<f32>().unwrap();

    let m = if a >= 1.0 {"High"} else if a >= 0.5 {"Middle"} else {"Low"};
    println!("{m}");
}