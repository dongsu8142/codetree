fn main() {
    let (mut a, mut b, c) = (5, 6, 7);
    (a, b) = (c, c);
    println!("{a} {b} {c}");
}