use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();

    let mut cnt_cl = 0;
    let mut cnt_co = 0;
    let mut cnt_to = 0;

    for i in 1..=n {
        if i % 12 == 0 {
            cnt_to += 1;
        } else if i % 3 == 0 {
            cnt_co += 1;
        } else if i % 2 == 0 {
            cnt_cl += 1;
        }
    }

    println!("{cnt_cl} {cnt_co} {cnt_to}")
}