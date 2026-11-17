fn main() {
    let (mut a, mut b) = (2, 5);
    (a, b) = (b, a);
    println!("{a}\n{b}");
}