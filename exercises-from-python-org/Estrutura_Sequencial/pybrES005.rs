use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que converta metros para centímetros
    
    println!("Digite os metros para converter");
    let convert = scan_int();
    println!("{} metros eh {} centimetros", convert, convert * 100);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
