use std::io;

fn main() {
    // https://wiki.python.org.br/EstruturaSequencial
    // Faça um Programa que peça a temperatura em graus Fahrenheit,
    // transforme e mostre a temperatura em graus Celsius.
    // C = 5 * ((F-32) / 9).

    println!("Digite a temperatura");
    let fahrenheit = scan_int();
    let celsius = 5 * ((fahrenheit - 32)/ 9);
    println!("{} Fahrenheit sao {} Celsius", fahrenheit, celsius);
}

fn scan_int()-> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();
    return number;
}
