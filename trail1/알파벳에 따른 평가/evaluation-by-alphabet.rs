use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let message = match input.trim() {
        "S" => "Superior",
        "A" => "Excellent",
        "B" => "Good",
        "C" => "Usually",
        "D" => "Effort",
        _ => "Failure"
    };

    println!("{message}");
}