use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let num = a.trim().parse::<f32>().unwrap();

    println!("{:.2}", num + 1.5);
}