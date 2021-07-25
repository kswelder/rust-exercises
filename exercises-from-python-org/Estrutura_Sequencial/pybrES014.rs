use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // 
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
