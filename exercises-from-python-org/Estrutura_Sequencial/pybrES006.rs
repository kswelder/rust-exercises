use std::io;

const PI:f64 = 3.14;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça o raio de um círculo,
    // calcule e mostre sua área.

    println!("Digite o raio");
    let raio = scan_int() as f64;
    let result = (PI * raio)* raio;
    println!("A area do circulo eh de {}", result);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
