fn main() {
    let (mut a, mut b, mut c) = (5, 6, 7);
    (a, b, c) = (c, a, b);
    println!("{a}\n{b}\n{c}");
}