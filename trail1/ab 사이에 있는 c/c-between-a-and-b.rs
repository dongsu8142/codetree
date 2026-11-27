use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    let mut satisfied = false;

    for i in a..=b {
        if i % c == 0 {
            satisfied = true;
            break;
        }
    }

    if satisfied {println!("YES")} else {println!("NO")}
}