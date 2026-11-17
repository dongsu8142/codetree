use std::io;

fn main() {
    let fit = 30.48;
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let num = a.trim().parse::<f32>().unwrap();

    println!("{:.1}", num * fit);
}