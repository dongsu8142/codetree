fn main() {
    let (mut a, mut b, c) = (1, 2, 3);
    (a, b) = (c, c);
    println!("{a} {b} {c}");
}