use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: u16 = input.trim().parse().unwrap();
    let mut cnt = 1u32;

    loop {
        if 2_u16.pow(cnt) == n {
            println!("{cnt}");
            break;
        }
        cnt += 1;
    }
}