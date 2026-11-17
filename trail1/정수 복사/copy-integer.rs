fn main() {
    let (a, mut b) = (3, 4);
    b = a;
    println!("{a} {b}\n{}", a * b)
}