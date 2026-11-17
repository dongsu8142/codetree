fn main() {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("입력을 읽는 데 실패했습니다.");
    
    let num: u8 = input
        .trim()
        .parse()
        .expect("올바른 숫자가 아닙니다.");
    
    println!("{}", num * 2)
}