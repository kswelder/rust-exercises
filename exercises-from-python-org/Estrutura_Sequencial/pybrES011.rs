use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça 2 números inteiros e um número real.
    // Calcule e mostre:

    // a) o produto do dobro do primeiro com metade do segundo .
    // b) a soma do triplo do primeiro com o terceiro.
    // c) o terceiro elevado ao cubo.

    println!("Digite um numero");
    let num1 = scan_int();
    println!("Digite o segundo numero");
    let num2 = scan_int();

    let mut num = String::new();
    println!("Digite um numero com casa decimal");
    io::stdin().read_line(&mut num).unwrap();

    let numf: f32 = num.trim().parse().unwrap();

    println!("primeiro: {}\nsegundo: {}\nterceiro: {}", num1, num2, numf);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
