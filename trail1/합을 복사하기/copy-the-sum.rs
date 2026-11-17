fn main() {
    let (mut a, mut b, mut c) = (1, 2, 3);
    let sum = a + b + c;
    (a, b, c) = (sum, sum, sum);
    println!("{a} {b} {c}");
}