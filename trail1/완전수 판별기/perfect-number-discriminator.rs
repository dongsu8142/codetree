use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();

    let mut sum = 1;
    let limit = n.isqrt();

    for i in 2..=limit {
        if n % i == 0 {
            sum += i;
            let pair = n / i;
            if i != pair {
                sum += pair;
            }
        }
    }

    if (sum == n) {
        println!("P")
    } else {
        println!("N")
    }
}