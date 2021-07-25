use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça as 4 notas bimestrais e mostre a média.

    println!("Digite as notas a seguir\nPrimeiro bimestre");
    let one = scan_int();
    println!("Segundo bimestre");
    let two = scan_int();
    println!("Terceiro bimestre");
    let tree = scan_int();
    println!("Quarto bimestre");
    let four = scan_int();
    let nota = (one + two + tree + four) / 4;
    println!("Sua media eh: {}", nota);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
