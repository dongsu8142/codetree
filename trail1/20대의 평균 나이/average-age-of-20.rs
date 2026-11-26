use std::io;

fn main() {
    let mut input = String::new();
    let mut cnt: usize = 0;
    let mut sum: usize = 0;

    loop {
        io::stdin().read_line(&mut input).unwrap();
        let age: u8 = input.trim().parse().unwrap();
        if age / 10 == 2 {
            sum += age as usize;
            cnt += 1;
        } else {
            println!("{:.2}", sum as f32 / cnt as f32);
            break;
        }
        input.clear()
    }
}